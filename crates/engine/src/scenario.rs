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

/// Execute a scenario non-interactively (forward-only, original behaviour).
pub async fn run_scenario(
    scenario: &Scenario,
    ctx: &AppContext,
    registry: &CommandRegistry,
) -> ScenarioResult {
    let total = scenario.steps.len();
    let mut results: HashMap<usize, StepOutcome> = HashMap::new();
    let mut overall = Status::Pass;

    let mut idx = 0;
    while idx < total {
        let step = &scenario.steps[idx];
        let label = step_label(step);
        let (result, expectation_met) = execute_step(step, idx, ctx, registry).await;

        if !expectation_met {
            overall = Status::Fail;
        }

        results.insert(
            idx,
            StepOutcome {
                label,
                status: if expectation_met {
                    StepStatus::Completed
                } else {
                    StepStatus::Failed
                },
                result,
            },
        );
        idx += 1;
    }

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

/// Execute a scenario interactively with go-back navigation.
///
/// `prompt_fn` is called at each step to ask the user whether to run, skip, or
/// go back. This keeps the engine crate free of direct terminal I/O
/// dependencies — the CLI crate provides the real prompter.
pub async fn run_scenario_interactive<F>(
    scenario: &Scenario,
    ctx: &AppContext,
    registry: &CommandRegistry,
    mut prompt_fn: F,
) -> ScenarioResult
where
    F: FnMut(usize, usize, &str, bool) -> Option<StepChoice>,
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
                idx = idx.saturating_sub(1);
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
            // Step failed — ask if user wants to continue
            let cont = prompt_fn(idx, total, &format!("{} (failed, continue?)", label), false);
            let status = if cont == Some(StepChoice::Run) {
                StepStatus::Failed
            } else {
                // User chose to abort or skip after failure
                results.insert(
                    idx,
                    StepOutcome {
                        label,
                        status: StepStatus::Failed,
                        result,
                    },
                );
                break;
            };
            results.insert(
                idx,
                StepOutcome {
                    label,
                    status,
                    result,
                },
            );
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
        // 3-step scenario: ping, ping, ping
        // Simulate: run step 0, go back at step 1, run step 0 again, run step 1, skip step 2
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

        // Track how many times prompt_fn is called and what idx it sees
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
                        Some(StepChoice::GoBack)
                    } // go back from step 1
                    2 => {
                        assert_eq!(idx, 0);
                        Some(StepChoice::Run)
                    } // re-run step 0
                    3 => {
                        assert_eq!(idx, 1);
                        Some(StepChoice::Run)
                    } // run step 1
                    4 => {
                        assert_eq!(idx, 2);
                        Some(StepChoice::Skip)
                    } // skip step 2
                    _ => panic!("unexpected call {}", n),
                }
            },
        )
        .await;

        assert_eq!(result.overall_status, Status::Pass);
        // 3 steps: step 0 re-run (pass), step 1 (pass), step 2 (skip)
        assert_eq!(result.step_results.len(), 3);
        assert_eq!(result.step_results[0].status, Status::Pass);
        assert_eq!(result.step_results[1].status, Status::Pass);
        assert_eq!(result.step_results[2].status, Status::Skip);
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
        )
        .await;

        assert_eq!(result.overall_status, Status::Pass);
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
                } // abort at step 1
            },
        )
        .await;

        assert_eq!(result.overall_status, Status::Pass);
        // Only step 0 completed, step 1 was never reached
        assert_eq!(result.step_results.len(), 1);
    }
}
