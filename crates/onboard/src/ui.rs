//! Small terminal-output helpers shared by every onboarding step.

use console::Style;

/// Print the top-level onboarding banner.
pub fn print_header() {
    let bold = Style::new().cyan().bold();
    println!();
    println!("{}", bold.apply_to("Tauri Template Onboarding"));
    println!(
        "{}",
        Style::new()
            .dim()
            .apply_to("  rename -> deps -> env -> hooks -> media")
    );
    println!();
}

/// Print a numbered step header.
pub fn print_step(n: usize, title: &str) {
    let style = Style::new().cyan().bold();
    println!();
    println!("{}", style.apply_to(format!("[{n}/5] {title}")));
}

/// Print a success line.
pub fn print_success(msg: &str) {
    println!(
        "  {} {}",
        Style::new().green().bold().apply_to("\u{2714}"),
        msg
    );
}

/// Print a warning line.
pub fn print_warning(msg: &str) {
    println!("  {} {}", Style::new().yellow().bold().apply_to("!"), msg);
}

/// Print a skip line.
pub fn print_skip(msg: &str) {
    println!(
        "  {} {}",
        Style::new().dim().apply_to("\u{2192}"),
        Style::new().dim().apply_to(msg)
    );
}

/// Print an error line.
pub fn print_error(msg: &str) {
    println!(
        "  {} {}",
        Style::new().red().bold().apply_to("\u{2717}"),
        msg
    );
}
