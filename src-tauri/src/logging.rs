use crate::global_config::get_config;
use regex::Regex;
use std::io;
use std::sync::{Arc, OnceLock};
use tracing::Level;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Layer};

static SESSION_ID: OnceLock<String> = OnceLock::new();

fn get_session_id() -> &'static str {
    SESSION_ID.get_or_init(|| {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    })
}

struct RedactingWriter<W> {
    inner: W,
    patterns: Arc<Vec<(Regex, String)>>,
    session_id: Option<Arc<String>>,
}

impl<W: io::Write> io::Write for RedactingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        let mut redacted = s.into_owned();

        // Prepend session ID if enabled
        if let Some(ref id) = self.session_id {
            // Only prepend to lines that aren't just whitespace/newlines
            if !redacted.trim().is_empty() {
                redacted = format!("[{}] {}", id, redacted);
            }
        }

        for (re, replacement) in self.patterns.iter() {
            if let std::borrow::Cow::Owned(s) = re.replace_all(&redacted, replacement) {
                redacted = s;
            }
        }
        self.inner.write_all(redacted.as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Clone)]
struct RedactingMakeWriter {
    patterns: Arc<Vec<(Regex, String)>>,
    session_id: Option<Arc<String>>,
}

impl<'a> fmt::MakeWriter<'a> for RedactingMakeWriter {
    type Writer = RedactingWriter<io::Stdout>;

    fn make_writer(&self) -> Self::Writer {
        RedactingWriter {
            inner: io::stdout(),
            patterns: self.patterns.clone(),
            session_id: self.session_id.clone(),
        }
    }
}

pub fn init_logging() {
    let config = get_config();

    // Determine the log level from config - pick the most verbose one enabled.
    // In a hierarchical system like tracing, the most verbose level (e.g., debug)
    // naturally includes all less verbose levels (e.g., info, warn, error).
    // We select the "widest" enabled threshold to ensure the user's request for
    // verbosity is honored even if multiple levels are checked.
    let level = if config.logging.levels.debug {
        "debug"
    } else if config.logging.levels.info {
        "info"
    } else if config.logging.levels.warning {
        "warn"
    } else if config.logging.levels.error || config.logging.levels.critical {
        "error"
    } else {
        "off"
    };

    // Use the level from config as the base filter.
    // Note: try_from_default_env() is skipped to ensure config is the source of truth.
    let filter = EnvFilter::new(level);

    // Setup redaction patterns
    let mut patterns = Vec::new();
    if config.logging.redaction.enabled {
        for p in &config.logging.redaction.patterns {
            match Regex::new(&p.regex) {
                Ok(re) => patterns.push((re, p.placeholder.clone())),
                Err(e) => eprintln!(
                    "Warning: Failed to compile redaction regex '{}': {}",
                    p.name, e
                ),
            }
        }
    }

    let patterns = Arc::new(patterns);

    let session_id = if config.logging.format.show_session_id {
        Some(Arc::new(get_session_id().to_string()))
    } else {
        None
    };

    let make_writer = RedactingMakeWriter {
        patterns,
        session_id,
    };

    // Base formatter configuration
    let location = &config.logging.format.location;
    let show_time = config.logging.format.show_time;

    // Helper to create a layer for a specific level with its location settings
    let make_layer = |level_filter: Level, show_location_for_level: bool| {
        // Location info is shown only if globally enabled AND enabled for this specific level
        let show = location.enabled && show_location_for_level;

        let layer = fmt::layer()
            .with_writer(make_writer.clone())
            .with_target(show && location.show_function)
            .with_file(show && location.show_file)
            .with_line_number(show && location.show_line)
            .with_thread_ids(false);

        // Apply time settings. We must box to unify types because without_time() changes the type.
        let layer = if !show_time {
            layer.without_time().boxed()
        } else {
            layer.boxed()
        };

        // Apply filter to restrict this layer to ONLY this level
        layer.with_filter(filter_fn(move |metadata| *metadata.level() == level_filter))
    };

    // Create separate layers for each level
    // Note: Trace level is not explicitly configured in LoggingLocationConfig, so we use Debug settings as fallback.
    let trace_layer = make_layer(Level::TRACE, location.show_for_debug);
    let debug_layer = make_layer(Level::DEBUG, location.show_for_debug);
    let info_layer = make_layer(Level::INFO, location.show_for_info);
    let warn_layer = make_layer(Level::WARN, location.show_for_warning);
    let error_layer = make_layer(Level::ERROR, location.show_for_error);

    // We collect layers into a Vec to ensure they all share the same Subscriber type <S>.
    // Chaining .with() causes S to change for each layer, which causes type mismatch
    // because make_layer closure infers a single concrete S.
    let layers = vec![
        trace_layer,
        debug_layer,
        info_layer,
        warn_layer,
        error_layer,
    ];

    tracing_subscriber::registry()
        .with(filter)
        .with(layers)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::Arc;

    #[test]
    fn test_session_id_generation() {
        let id1 = get_session_id();
        let id2 = get_session_id();
        assert_eq!(id1, id2);
        assert_eq!(id1.len(), 8);
        assert!(id1.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_redacting_writer_functionality() {
        let mut buffer = Vec::new();
        let patterns = vec![(
            Regex::new(r"password=\w+").unwrap(),
            "password=***".to_string(),
        )];
        let patterns = Arc::new(patterns);

        let mut writer = RedactingWriter {
            inner: &mut buffer,
            patterns,
            session_id: Some(Arc::new("TESTID".to_string())),
        };

        let input = b"login password=secret";
        writer.write_all(input).unwrap();
        writer.flush().unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("[TESTID]"));
        assert!(output.contains("password=***"));
        assert!(!output.contains("secret"));
    }
}
