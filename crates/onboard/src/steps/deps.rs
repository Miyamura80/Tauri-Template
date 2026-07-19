//! Step 2: install project dependencies (bun packages + cargo registry fetch).

use std::path::Path;
use std::process::Command;

use dialoguer::{theme::ColorfulTheme, Confirm};

use super::{have, StepResult};
use crate::ui;

fn run_cmd(project_root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(project_root)
        .status()
        .map_err(|e| format!("failed to launch `{program}`: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "`{program} {}` exited with {status}",
            args.join(" ")
        ))
    }
}

pub fn run(project_root: &Path) -> StepResult {
    ui::print_step(2, "Install dependencies");

    let proceed = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Install Node and Rust dependencies now?")
        .default(true)
        .interact()
    {
        Ok(v) => v,
        Err(e) => return StepResult::Failed(format!("Prompt error: {e}")),
    };
    if !proceed {
        ui::print_skip("Skipped dependency installation.");
        return StepResult::Skipped;
    }

    let mut did_something = false;

    if have("bun") {
        println!("  Running `bun install`...");
        if let Err(e) = run_cmd(project_root, "bun", &["install"]) {
            return StepResult::Failed(e);
        }
        ui::print_success("Node dependencies installed");
        did_something = true;
    } else {
        ui::print_warning("bun not found; skipping `bun install` (install from https://bun.sh).");
    }

    if have("cargo") {
        println!("  Running `cargo fetch`...");
        if let Err(e) = run_cmd(project_root, "cargo", &["fetch"]) {
            return StepResult::Failed(e);
        }
        ui::print_success("Rust dependencies fetched");
        did_something = true;
    } else {
        ui::print_warning(
            "cargo not found; skipping `cargo fetch` (install from https://rustup.rs).",
        );
    }

    if did_something {
        StepResult::Success
    } else {
        StepResult::Skipped
    }
}
