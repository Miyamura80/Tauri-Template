//! Onboarding steps. Each step exposes a `run(project_root) -> StepResult`.

pub mod deps;
pub mod env;
pub mod hooks;
pub mod media;
pub mod rename;

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
