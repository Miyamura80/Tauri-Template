//! Step 1: rename the template sentinels to the new project's identity.
//!
//! Touches `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`,
//! and `README.md`. The pure `rename_*` helpers do the string surgery so they
//! can be unit-tested without a live filesystem or a TTY.

use std::fs;
use std::path::Path;

use dialoguer::{theme::ColorfulTheme, Input};

use super::StepResult;
use crate::ui;

/// Turn a free-form project name into a slug safe for package names and the
/// reverse-DNS identifier segment: lowercase, non-alphanumeric collapsed to
/// single hyphens, trimmed.
pub fn slugify(name: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in name.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

/// Sanitize a segment for a reverse-DNS identifier (letters/digits only).
fn ident_segment(s: &str) -> String {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    if cleaned.is_empty() {
        "app".to_string()
    } else {
        cleaned
    }
}

/// Build the Tauri bundle identifier `com.<owner>.<name>`.
pub fn build_identifier(owner: &str, name_slug: &str) -> String {
    format!("com.{}.{}", ident_segment(owner), name_slug)
}

pub fn rename_package_json(content: &str, new_name: &str) -> String {
    content.replace(
        "\"name\": \"tauri-app\"",
        &format!("\"name\": \"{new_name}\""),
    )
}

pub fn rename_tauri_conf(content: &str, new_name: &str, identifier: &str) -> String {
    content
        .replace(
            "\"productName\": \"tauri-app\"",
            &format!("\"productName\": \"{new_name}\""),
        )
        .replace(
            "\"identifier\": \"com.eito.tauri-app\"",
            &format!("\"identifier\": \"{identifier}\""),
        )
        .replace(
            "\"title\": \"tauri-app\"",
            &format!("\"title\": \"{new_name}\""),
        )
}

/// Rename the crate in `src-tauri/Cargo.toml`. Updates both `name` and
/// `default-run` so the default binary still resolves after the rename.
pub fn rename_cargo_toml(content: &str, new_name: &str) -> String {
    content
        .replace("name = \"tauri-app\"", &format!("name = \"{new_name}\""))
        .replace(
            "default-run = \"tauri-app\"",
            &format!("default-run = \"{new_name}\""),
        )
}

pub fn rename_readme(content: &str, title: &str, description: &str) -> String {
    let mut out = content.replace("# Tauri-Template", &format!("# {title}"));
    if !description.is_empty() {
        out = out.replace(
            "<b>agent ready tauri template</b>",
            &format!("<b>{description}</b>"),
        );
    }
    out
}

fn apply_file<F>(
    project_root: &Path,
    rel: &str,
    f: F,
    changed: &mut Vec<String>,
) -> Result<(), String>
where
    F: Fn(&str) -> String,
{
    let path = project_root.join(rel);
    if !path.exists() {
        ui::print_warning(&format!("{rel} not found, skipping"));
        return Ok(());
    }
    let content = fs::read_to_string(&path).map_err(|e| format!("read {rel}: {e}"))?;
    let updated = f(&content);
    if updated != content {
        fs::write(&path, updated).map_err(|e| format!("write {rel}: {e}"))?;
        changed.push(rel.to_string());
    }
    Ok(())
}

pub fn run(project_root: &Path) -> StepResult {
    ui::print_step(1, "Rename project");

    let name: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name (leave empty to skip rename)")
        .allow_empty(true)
        .interact_text()
    {
        Ok(v) => v,
        Err(e) => return StepResult::Failed(format!("Input error: {e}")),
    };

    let slug = slugify(&name);
    if slug.is_empty() {
        ui::print_skip("No name entered; leaving template sentinels in place.");
        return StepResult::Skipped;
    }

    let description: String = match Input::with_theme(&ColorfulTheme::default())
        .with_prompt("One-line description")
        .default(String::new())
        .allow_empty(true)
        .interact_text()
    {
        Ok(v) => v,
        Err(e) => return StepResult::Failed(format!("Input error: {e}")),
    };

    let owner = std::env::var("USER").unwrap_or_else(|_| "app".to_string());
    let identifier = build_identifier(&owner, &slug);
    println!("  bundle identifier: {identifier}");

    let mut changed: Vec<String> = Vec::new();
    let title = name.trim().to_string();

    if let Err(e) = apply_file(
        project_root,
        "package.json",
        |c| rename_package_json(c, &slug),
        &mut changed,
    ) {
        return StepResult::Failed(e);
    }
    if let Err(e) = apply_file(
        project_root,
        "src-tauri/tauri.conf.json",
        |c| rename_tauri_conf(c, &slug, &identifier),
        &mut changed,
    ) {
        return StepResult::Failed(e);
    }
    if let Err(e) = apply_file(
        project_root,
        "src-tauri/Cargo.toml",
        |c| rename_cargo_toml(c, &slug),
        &mut changed,
    ) {
        return StepResult::Failed(e);
    }
    if let Err(e) = apply_file(
        project_root,
        "README.md",
        |c| rename_readme(c, &title, &description),
        &mut changed,
    ) {
        return StepResult::Failed(e);
    }

    if changed.is_empty() {
        ui::print_skip("Sentinels already renamed; nothing to change.");
        StepResult::Skipped
    } else {
        for c in &changed {
            ui::print_success(&format!("updated {c}"));
        }
        StepResult::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_normalizes() {
        assert_eq!(slugify("My Cool App"), "my-cool-app");
        assert_eq!(slugify("  Spaces  "), "spaces");
        assert_eq!(slugify("weird__name!!"), "weird-name");
        assert_eq!(slugify("---"), "");
    }

    #[test]
    fn identifier_is_reverse_dns() {
        assert_eq!(
            build_identifier("Miyamura80", "my-app"),
            "com.miyamura80.my-app"
        );
        assert_eq!(build_identifier("", "app"), "com.app.app");
    }

    #[test]
    fn package_json_rename() {
        let src = "{ \"name\": \"tauri-app\" }";
        assert_eq!(
            rename_package_json(src, "my-app"),
            "{ \"name\": \"my-app\" }"
        );
    }

    #[test]
    fn cargo_toml_rename_updates_default_run() {
        let src = "name = \"tauri-app\"\ndefault-run = \"tauri-app\"\n";
        let out = rename_cargo_toml(src, "my-app");
        assert!(out.contains("name = \"my-app\""));
        assert!(out.contains("default-run = \"my-app\""));
    }

    #[test]
    fn tauri_conf_rename() {
        let src = "\"productName\": \"tauri-app\",\n\"identifier\": \"com.eito.tauri-app\",\n\"title\": \"tauri-app\",";
        let out = rename_tauri_conf(src, "my-app", "com.me.my-app");
        assert!(out.contains("\"productName\": \"my-app\""));
        assert!(out.contains("\"identifier\": \"com.me.my-app\""));
        assert!(out.contains("\"title\": \"my-app\""));
    }

    #[test]
    fn readme_rename() {
        let src = "# Tauri-Template\n<b>agent ready tauri template</b>";
        let out = rename_readme(src, "My App", "does things");
        assert!(out.contains("# My App"));
        assert!(out.contains("<b>does things</b>"));
    }
}
