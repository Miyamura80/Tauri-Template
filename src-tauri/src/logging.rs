use crate::config::get_config;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging() {
    let config = get_config();

    // Determine the log level from config
    let mut level = "off";
    if config.logging.levels.critical {
        level = "error";
    }
    if config.logging.levels.error {
        level = "error";
    }
    if config.logging.levels.warning {
        level = "warn";
    }
    if config.logging.levels.info {
        level = "info";
    }
    if config.logging.levels.debug {
        level = "debug";
    }

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    let builder = fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(config.logging.format.location.show_file)
        .with_file(config.logging.format.location.show_file)
        .with_line_number(config.logging.format.location.show_line)
        .with_thread_ids(false);

    let builder = if !config.logging.format.show_time {
        builder.without_time()
    } else {
        builder
    };

    builder.init();

    // Note: Redaction patterns from config.logging.redaction are not yet implemented
    // in this tracing setup. This requires a custom tracing Layer.
}
