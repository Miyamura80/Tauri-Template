//! Onboarding steps. Each step exposes a `run(project_root) -> StepResult`.

use std::process::Command;

pub mod deps;
pub mod env;
pub mod hooks;
pub mod media;
pub mod rename;

/// Return `true` if `bin` is on `PATH` and answers `--version` successfully.
/// Shared by the steps that gate work on an external tool being installed.
pub(crate) fn have(bin: &str) -> bool {
    Command::new(bin)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Outcome of a single onboarding step.
pub enum StepResult {
    /// The step ran and made (or confirmed) changes.
    Success,
    /// The step was intentionally skipped (missing prerequisite or user choice).
    Skipped,
    /// The step failed; the string carries a human-readable reason.
    Failed(String),
}

impl StepResult {
    /// Short label for the end-of-run summary.
    pub fn label(&self) -> &'static str {
        match self {
            StepResult::Success => "done",
            StepResult::Skipped => "skipped",
            StepResult::Failed(_) => "failed",
        }
    }
}
