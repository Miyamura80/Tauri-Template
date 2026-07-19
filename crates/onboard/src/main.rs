//! Interactive onboarding CLI for the Tauri template.
//!
//! Walks a new project through the one-time setup: rename the template
//! sentinels, install dependencies, seed `.env`, enable git hooks, and
//! optionally generate media assets. Each step is independent and degrades
//! gracefully when a prerequisite (a tool, a file, an API key) is missing.
//!
//! Run headlessly-safe: any step whose prompt cannot run simply reports a
//! failure for that step without aborting the others.

mod steps;
mod ui;

use std::path::{Path, PathBuf};

use steps::StepResult;

/// Locate the project root by walking up from `start` until we find a directory
/// that looks like this template (has `src-tauri/` and `package.json`).
fn find_project_root(start: &Path) -> PathBuf {
    let mut dir = start;
    loop {
        if dir.join("src-tauri").is_dir() && dir.join("package.json").is_file() {
            return dir.to_path_buf();
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return start.to_path_buf(),
        }
    }
}

fn main() {
    // Optional first positional arg overrides the project root.
    let arg_root = std::env::args().nth(1).map(PathBuf::from);
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let project_root = match arg_root {
        Some(r) => r,
        None => find_project_root(&cwd),
    };

    ui::print_header();
    println!(
        "  {} {}",
        console::Style::new().dim().apply_to("project root:"),
        project_root.display()
    );

    let results: Vec<(&str, StepResult)> = vec![
        ("rename", steps::rename::run(&project_root)),
        ("deps", steps::deps::run(&project_root)),
        ("env", steps::env::run(&project_root)),
        ("hooks", steps::hooks::run(&project_root)),
        ("media", steps::media::run(&project_root)),
    ];

    println!();
    println!(
        "{}",
        console::Style::new().cyan().bold().apply_to("Summary")
    );
    let mut failures = 0;
    for (name, result) in &results {
        match result {
            StepResult::Success => ui::print_success(&format!("{name}: {}", result.label())),
            StepResult::Skipped => ui::print_skip(&format!("{name}: {}", result.label())),
            StepResult::Failed(reason) => {
                failures += 1;
                ui::print_error(&format!("{name}: {} ({reason})", result.label()));
            }
        }
    }

    println!();
    if failures == 0 {
        ui::print_success("Onboarding complete.");
    } else {
        ui::print_warning(&format!(
            "Onboarding finished with {failures} failed step(s). Re-run `make onboard` to retry."
        ));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_project_root_walks_up() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("src-tauri")).unwrap();
        std::fs::write(root.join("package.json"), "{}").unwrap();
        let nested = root.join("crates/onboard/src");
        std::fs::create_dir_all(&nested).unwrap();
        assert_eq!(find_project_root(&nested), root.to_path_buf());
    }

    #[test]
    fn find_project_root_falls_back_to_start() {
        let tmp = tempfile::tempdir().unwrap();
        let start = tmp.path().join("nowhere");
        std::fs::create_dir_all(&start).unwrap();
        assert_eq!(find_project_root(&start), start);
    }
}
