//! Step 4: enable git hooks with prek.

use std::path::Path;
use std::process::Command;

use dialoguer::{theme::ColorfulTheme, Confirm};

use super::{have, StepResult};
use crate::ui;

pub fn run(project_root: &Path) -> StepResult {
    ui::print_step(4, "Enable git hooks (prek)");

    if !project_root.join(".git").exists() {
        ui::print_warning("Not a git repository; skipping hook installation.");
        return StepResult::Skipped;
    }

    let proceed = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Install prek git hooks now?")
        .default(true)
        .interact()
    {
        Ok(v) => v,
        Err(e) => return StepResult::Failed(format!("Prompt error: {e}")),
    };
    if !proceed {
        ui::print_skip("Skipped hook installation.");
        return StepResult::Skipped;
    }

    if !have("prek") {
        ui::print_warning(
            "prek not found. Install it with `bun add -g prek`, then run `prek install`.",
        );
        return StepResult::Skipped;
    }

    let status = Command::new("prek")
        .arg("install")
        .current_dir(project_root)
        .status();

    match status {
        Ok(s) if s.success() => {
            ui::print_success("prek hooks installed");
            StepResult::Success
        }
        Ok(s) => StepResult::Failed(format!("`prek install` exited with {s}")),
        Err(e) => StepResult::Failed(format!("failed to launch prek: {e}")),
    }
}
