//! Scenario runner – execute scripted flows from YAML files.

use crate::commands::CommandRegistry;
use crate::context::AppContext;
use crate::probes;
use crate::types::*;
use std::collections::HashMap;

/// Load a scenario from a YAML string.
pub fn load_scenario(yaml: &str) -> Result<Scenario, String> {
    serde_yaml::from_str(yaml).map_err(|e| format!("failed to parse scenario YAML: {}", e))
}

/// User choice at each interactive step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepChoice {
    Run,
    Skip,
    GoBack,
}

/// User choice after a step failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureChoice {
    Continue,
    Abort,
}

/// Outcome stored per step so re-running overwrites the previous result.
#[derive(Debug, Clone)]
pub struct StepOutcome {
    pub label: String,
    pub status: StepStatus,
    pub result: CommandResult,
}

/// Disposition of a completed step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepStatus {
    Completed,
    Skipped,
    Failed,
}

/// Return the label (target name) for a scenario step.
fn step_label(step: &ScenarioStep) -> String {
    match step {
        ScenarioStep::Call { call, .. } => call.clone(),
        ScenarioStep::Probe { probe } => format!("probe:{}", probe),
    }
}

/// Execute a single scenario step and return the result plus whether the
/// expectation was met.
async fn execute_step(
    step: &ScenarioStep,
    idx: usize,
    ctx: &AppContext,
    registry: &CommandRegistry,
) -> (CommandResult, bool) {
    match step {
        ScenarioStep::Call {
            call,
            args,
            expect_status,
            timeout_ms: _timeout_ms,
        } => {
            // TODO: honour timeout_ms with tokio::time::timeout
            let r = registry.execute(call, args.clone(), ctx);
            let actual_status = serde_json::to_value(r.status)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default();
            let met = actual_status == *expect_status;
            if !met {
                tracing::warn!(
                    step = idx,
                    expected = %expect_status,
                    actual = %actual_status,
                    "scenario step status mismatch"
                );
            }
            (r, met)
        }
        ScenarioStep::Probe { probe } => {
            let r = probes::run_probe(probe, ctx).await;
            let met = r.status == Status::Pass || r.status == Status::Skip;
            (r, met)
        }
    }
}

/// Execute a scenario non-interactively (forward-only).
pub async fn run_scenario(
    scenario: &Scenario,
    ctx: &AppContext,
    registry: &CommandRegistry,
) -> ScenarioResult {
    let mut step_results = Vec::new();
    let mut overall = Status::Pass;

    for (i, step) in scenario.steps.iter().enumerate() {
        let (result, expectation_met) = execute_step(step, i, ctx, registry).await;
        if !expectation_met {
            overall = Status::Fail;
        }
        step_results.push(result);
    }

    ScenarioResult {
        name: scenario.name.clone(),
        overall_status: overall,
        step_results,
    }
}

/// Execute a scenario interactively with go-back navigation.
///
/// - `prompt_fn` is called at each step to ask the user whether to run, skip,
///   or go back. Returns `None` to abort the scenario.
/// - `failure_fn` is called when a step fails, asking the user whether to
///   continue or abort. Returns `None` to abort.
///
/// This keeps the engine crate free of direct terminal I/O dependencies —
/// the CLI crate provides the real prompters.
pub async fn run_scenario_interactive<F, G>(
    scenario: &Scenario,
    ctx: &AppContext,
    registry: &CommandRegistry,
    mut prompt_fn: F,
    mut failure_fn: G,
) -> ScenarioResult
where
    F: FnMut(usize, usize, &str, bool) -> Option<StepChoice>,
    G: FnMut(usize, usize, &str) -> Option<FailureChoice>,
{
    let total = scenario.steps.len();
    let mut results: HashMap<usize, StepOutcome> = HashMap::new();

    let mut idx = 0;
    while idx < total {
        let step = &scenario.steps[idx];
        let label = step_label(step);
        let can_go_back = idx > 0;

        let choice = match prompt_fn(idx, total, &label, can_go_back) {
            Some(c) => c,
            None => break, // user aborted
        };

        match choice {
            StepChoice::GoBack => {
                // Use an explicit guard rather than saturating_sub so the
                // intent — stay at step 0 when can_go_back is false — is
                // immediately visible without reading clippy docs.
                #[allow(clippy::implicit_saturating_sub)]
                if idx > 0 {
                    // Invalidate all steps at or after the new position so
                    // stale results from the previous forward pass cannot
                    // masquerade as a completed run if the user later aborts.
                    for stale in idx..total {
                        results.remove(&stale);
                    }
                    idx -= 1;
                }
                continue;
            }
            StepChoice::Skip => {
                let run_id = new_run_id();
                results.insert(
                    idx,
                    StepOutcome {
                        label: label.clone(),
                        status: StepStatus::Skipped,
                        result: result_skip("scenario", &label, &run_id, 0, "user skipped"),
                    },
                );
                idx += 1;
                continue;
            }
            StepChoice::Run => {}
        }

        let (result, expectation_met) = execute_step(step, idx, ctx, registry).await;

        if !expectation_met {
            // Step failed — ask if user wants to continue or abort
            let decision = failure_fn(idx, total, &label);
            results.insert(
                idx,
                StepOutcome {
                    label,
                    status: StepStatus::Failed,
                    result,
                },
            );
            if decision != Some(FailureChoice::Continue) {
                break;
            }
        } else {
            results.insert(
                idx,
                StepOutcome {
                    label,
                    status: StepStatus::Completed,
                    result,
                },
            );
        }

        idx += 1;
    }

    // Derive overall status from results
    let overall = if results.values().any(|o| o.status == StepStatus::Failed) {
        Status::Fail
    } else if results.len() < total
        || (total > 0 && results.values().all(|o| o.status == StepStatus::Skipped))
    {
        // User aborted before all steps were reached, or skipped every step
        Status::Skip
    } else {
        Status::Pass
    };

    // Collect results in step order
    let step_results: Vec<CommandResult> = (0..total)
        .filter_map(|i| results.remove(&i).map(|o| o.result))
        .collect();

    ScenarioResult {
        name: scenario.name.clone(),
        overall_status: overall,
        step_results,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scenario() {
        let yaml = r#"
name: basic test
steps:
  - call: "ping"
    args: {}
    expect_status: "pass"
    timeout_ms: 5000
  - probe: "filesystem"
"#;
        let s = load_scenario(yaml).expect("should parse");
        assert_eq!(s.name, Some("basic test".into()));
        assert_eq!(s.steps.len(), 2);
    }

    #[tokio::test]
    async fn test_run_scenario_ping() {
        let yaml = r#"
steps:
  - call: "ping"
    args: {}
    expect_status: "pass"
"#;
        let scenario = load_scenario(yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();
        let result = run_scenario(&scenario, &ctx, &reg).await;
        assert_eq!(result.overall_status, Status::Pass);
        assert_eq!(result.step_results.len(), 1);
    }

    #[test]
    fn test_parse_scenario_minimal() {
        let yaml = r#"
steps:
  - call: "read_file"
    args:
      path: "/tmp/nope"
"#;
        let s = load_scenario(yaml).expect("should parse");
        assert_eq!(s.steps.len(), 1);
    }

    #[tokio::test]
    async fn test_interactive_go_back() {
        // 3-step scenario: write_file (step 0), ping, ping
        // Simulate: run step 0, go back at step 1, run step 0 again, run step 1, skip step 2
        // Use write_file for step 0 so we can verify it actually executed twice
        // by checking the file exists (proves re-execution, not caching).
        let tmp = std::env::temp_dir().join("engine_test_go_back_exec.txt");
        let tmp_str = tmp.to_str().unwrap().to_string();
        // Clean up from any prior run
        let _ = std::fs::remove_file(&tmp);

        let yaml = format!(
            r#"
steps:
  - call: "write_file"
    args:
      path: "{}"
      content: "x"
    expect_status: "pass"
  - call: "ping"
    args: {{}}
    expect_status: "pass"
  - call: "ping"
    args: {{}}
    expect_status: "pass"
"#,
            tmp_str
        );
        let scenario = load_scenario(&yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();

        let call_count = std::cell::Cell::new(0usize);
        let result = run_scenario_interactive(
            &scenario,
            &ctx,
            &reg,
            |idx, _total, _label, _can_go_back| {
                let n = call_count.get();
                call_count.set(n + 1);
                match n {
                    0 => {
                        assert_eq!(idx, 0);
                        Some(StepChoice::Run)
                    }
                    1 => {
                        assert_eq!(idx, 1);
                        // Delete the file before going back, so re-execution
                        // of step 0 must recreate it.
                        std::fs::remove_file(&tmp).expect("file should exist after first run");
                        Some(StepChoice::GoBack)
                    }
                    2 => {
                        assert_eq!(idx, 0);
                        Some(StepChoice::Run)
                    }
                    3 => {
                        assert_eq!(idx, 1);
                        Some(StepChoice::Run)
                    }
                    4 => {
                        assert_eq!(idx, 2);
                        Some(StepChoice::Skip)
                    }
                    _ => panic!("unexpected call {}", n),
                }
            },
            |_idx, _total, _label| panic!("no failures expected"),
        )
        .await;

        assert_eq!(result.overall_status, Status::Pass);
        assert_eq!(result.step_results.len(), 3);
        assert_eq!(result.step_results[0].status, Status::Pass);
        assert_eq!(result.step_results[1].status, Status::Pass);
        assert_eq!(result.step_results[2].status, Status::Skip);

        // Verify step 0 actually re-executed (file recreated after deletion)
        assert!(tmp.exists(), "write_file should have been called twice");
        let _ = std::fs::remove_file(&tmp);
    }

    #[tokio::test]
    async fn test_interactive_skip_all() {
        let yaml = r#"
steps:
  - call: "ping"
    args: {}
    expect_status: "pass"
  - call: "ping"
    args: {}
    expect_status: "pass"
"#;
        let scenario = load_scenario(yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();

        let result = run_scenario_interactive(
            &scenario,
            &ctx,
            &reg,
            |_idx, _total, _label, _can_go_back| Some(StepChoice::Skip),
            |_idx, _total, _label| panic!("no failures expected"),
        )
        .await;

        assert_eq!(result.overall_status, Status::Skip);
        assert_eq!(result.step_results.len(), 2);
        assert_eq!(result.step_results[0].status, Status::Skip);
        assert_eq!(result.step_results[1].status, Status::Skip);
    }

    #[tokio::test]
    async fn test_interactive_abort() {
        let yaml = r#"
steps:
  - call: "ping"
    args: {}
    expect_status: "pass"
  - call: "ping"
    args: {}
    expect_status: "pass"
"#;
        let scenario = load_scenario(yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();

        let result = run_scenario_interactive(
            &scenario,
            &ctx,
            &reg,
            |idx, _total, _label, _can_go_back| {
                if idx == 0 {
                    Some(StepChoice::Run)
                } else {
                    None
                }
            },
            |_idx, _total, _label| panic!("no failures expected"),
        )
        .await;

        assert_eq!(result.overall_status, Status::Skip);
        assert_eq!(result.step_results.len(), 1);
    }

    #[tokio::test]
    async fn test_interactive_failure_continue() {
        // Step 0 passes, step 1 fails (expect "fail" but ping returns "pass"),
        // user continues, step 2 passes
        let yaml = r#"
steps:
  - call: "ping"
    args: {}
    expect_status: "pass"
  - call: "ping"
    args: {}
    expect_status: "fail"
  - call: "ping"
    args: {}
    expect_status: "pass"
"#;
        let scenario = load_scenario(yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();

        let result = run_scenario_interactive(
            &scenario,
            &ctx,
            &reg,
            |_idx, _total, _label, _can_go_back| Some(StepChoice::Run),
            |idx, _total, _label| {
                assert_eq!(idx, 1); // only step 1 should fail
                Some(FailureChoice::Continue)
            },
        )
        .await;

        assert_eq!(result.overall_status, Status::Fail);
        assert_eq!(result.step_results.len(), 3);
        assert_eq!(result.step_results[0].status, Status::Pass);
        // step 1's CommandResult.status is Pass (ping succeeded) but its expectation
        // was "fail" — the only externally visible evidence is overall_status above.
        assert_eq!(result.step_results[1].status, Status::Pass);
        // step 1 failed expectation but we continued to step 2
        assert_eq!(result.step_results[2].status, Status::Pass);
    }

    #[tokio::test]
    async fn test_interactive_failure_abort() {
        // Step 0 fails (expect "fail" but ping returns "pass"), user aborts
        let yaml = r#"
steps:
  - call: "ping"
    args: {}
    expect_status: "fail"
  - call: "ping"
    args: {}
    expect_status: "pass"
"#;
        let scenario = load_scenario(yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();

        let result = run_scenario_interactive(
            &scenario,
            &ctx,
            &reg,
            |_idx, _total, _label, _can_go_back| Some(StepChoice::Run),
            |idx, _total, _label| {
                assert_eq!(idx, 0);
                Some(FailureChoice::Abort)
            },
        )
        .await;

        assert_eq!(result.overall_status, Status::Fail);
        // Only step 0 recorded, step 1 never reached
        assert_eq!(result.step_results.len(), 1);
    }

    #[tokio::test]
    async fn test_interactive_go_back_invalidates_stale_results() {
        // Run step 0, skip step 1, go back from step 2, go back from step 1,
        // then abort. The go-back should invalidate the stale Skip for step 1,
        // so the result only contains step 0 and overall is Skip (not Pass).
        let yaml = r#"
steps:
  - call: "ping"
    args: {}
    expect_status: "pass"
  - call: "ping"
    args: {}
    expect_status: "pass"
  - call: "ping"
    args: {}
    expect_status: "pass"
"#;
        let scenario = load_scenario(yaml).unwrap();
        let ctx = AppContext::default_headless();
        let reg = CommandRegistry::new();

        let call_count = std::cell::Cell::new(0usize);
        let result = run_scenario_interactive(
            &scenario,
            &ctx,
            &reg,
            |idx, _total, _label, _can_go_back| {
                let n = call_count.get();
                call_count.set(n + 1);
                match n {
                    0 => {
                        assert_eq!(idx, 0);
                        Some(StepChoice::Run)
                    } // run step 0
                    1 => {
                        assert_eq!(idx, 1);
                        Some(StepChoice::Skip)
                    } // skip step 1
                    2 => {
                        assert_eq!(idx, 2);
                        Some(StepChoice::GoBack)
                    } // go back to step 1
                    3 => {
                        assert_eq!(idx, 1);
                        Some(StepChoice::GoBack)
                    } // go back to step 0
                    4 => {
                        assert_eq!(idx, 0);
                        None
                    } // abort
                    _ => panic!("unexpected call {}", n),
                }
            },
            |_idx, _total, _label| panic!("no failures expected"),
        )
        .await;

        // Go-back invalidated the stale Skip for step 1, so only step 0 remains.
        assert_eq!(result.overall_status, Status::Skip);
        assert_eq!(result.step_results.len(), 1);
        assert_eq!(result.step_results[0].status, Status::Pass);
    }
}
