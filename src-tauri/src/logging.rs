use crate::config::get_config;
use tracing_subscriber::EnvFilter;

pub fn init_logging() {
    let config = get_config();

    // Set up filter based on config levels
    // This is a basic implementation mapping boolean flags to tracing levels
    // A more complex implementation could fine-tune per module
    let mut level = "info";
    if config.logging.levels.debug {
        level = "debug";
    }

    // If specific levels are disabled, we might need a more complex filter construction
    // For now, we rely on the primary level

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    let builder = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(config.logging.format.location.show_file)
        .with_thread_ids(false) // config doesn't have a setting for this, default off
        .with_file(config.logging.format.location.show_file)
        .with_line_number(config.logging.format.location.show_line);

    // Apply time formatting
    if !config.logging.format.show_time {
        builder.without_time().init();
    } else {
        builder.init();
    }
}
