use crate::global_config::{get_config, AppConfig};
use regex::Regex;
use std::io;
use std::sync::{Arc, OnceLock};
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
        let mut redacted = s;

        // Prepend session ID if enabled
        if let Some(ref id) = self.session_id {
            // Only prepend to lines that aren't just whitespace/newlines
            if !redacted.trim().is_empty() {
                redacted = std::borrow::Cow::Owned(format!("[{}] {}", id, redacted));
            }
        }

        for (re, replacement) in self.patterns.iter() {
            if let std::borrow::Cow::Owned(s) = re.replace_all(&redacted, replacement) {
                redacted = std::borrow::Cow::Owned(s);
            }
        }
        self.inner.write_all(redacted.as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

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

fn determine_log_level(config: &AppConfig) -> &'static str {
    // Determine the log level from config - pick the most verbose one enabled.
    // In a hierarchical system like tracing, the most verbose level (e.g., debug)
    // naturally includes all less verbose levels (e.g., info, warn, error).
    // We select the "widest" enabled threshold to ensure the user's request for
    // verbosity is honored even if multiple levels are checked.
    if config.logging.verbose || config.logging.levels.debug {
        "debug"
    } else if config.logging.levels.info {
        "info"
    } else if config.logging.levels.warning {
        "warn"
    } else if config.logging.levels.error || config.logging.levels.critical {
        "error"
    } else {
        "off"
    }
}

pub fn init_logging() {
    let config = get_config();
    let level = determine_log_level(config);

    // Use the level from config as the base filter.
    // Note: try_from_default_env() is skipped to ensure config is the source of truth.
    let filter = EnvFilter::new(level);

    // Base formatter configuration
    let location = &config.logging.format.location;
    let show_file = location.show_file;
    let show_line = location.show_line;
    let show_target = location.show_function; // Map show_function to tracing's target display

    // TODO: Implement per-level location display control (show_for_info, show_for_debug, etc.)
    // in Phase 4. Currently, location settings are applied globally if enabled.
    // This requires separate layers for each level using with_filter().

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

    // Use Layer::boxed() to unify the types of the if/else branches
    let fmt_layer = if !config.logging.format.show_time {
        fmt::layer()
            .with_writer(make_writer)
            .with_target(show_target)
            .with_file(show_file)
            .with_line_number(show_line)
            .with_thread_ids(false)
            .without_time()
            .boxed()
    } else {
        fmt::layer()
            .with_writer(make_writer)
            .with_target(show_target)
            .with_file(show_file)
            .with_line_number(show_line)
            .with_thread_ids(false)
            .boxed()
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
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

    #[test]
    fn test_determine_log_level() {
        use crate::global_config::*;
        use std::collections::HashMap;

        // Helper to create a default config for testing
        fn create_test_config() -> AppConfig {
            AppConfig {
                model_name: "test".into(),
                dot_global_config_health_check: true,
                dev_env: "test".into(),
                example_parent: ExampleParent {
                    example_child: "val".into(),
                },
                default_llm: DefaultLlm {
                    default_model: "test".into(),
                    fallback_model: None,
                    default_temperature: 0.5,
                    default_max_tokens: 100,
                },
                llm_config: LlmConfig {
                    cache_enabled: false,
                    retry: RetryConfig {
                        max_attempts: 1,
                        min_wait_seconds: 1,
                        max_wait_seconds: 1,
                    },
                },
                logging: LoggingConfig {
                    verbose: false,
                    format: LoggingFormatConfig {
                        show_time: false,
                        show_session_id: false,
                        location: LoggingLocationConfig {
                            enabled: false,
                            show_file: false,
                            show_function: false,
                            show_line: false,
                            show_for_info: false,
                            show_for_debug: false,
                            show_for_warning: false,
                            show_for_error: false,
                        },
                    },
                    levels: LoggingLevelsConfig {
                        debug: false,
                        info: false,
                        warning: false,
                        error: false,
                        critical: false,
                    },
                    redaction: RedactionConfig::default(),
                },
                features: HashMap::new(),
                openai_api_key: None,
                anthropic_api_key: None,
                groq_api_key: None,
                perplexity_api_key: None,
                gemini_api_key: None,
            }
        }

        let mut config = create_test_config();

        // 1. Verbose true -> debug
        config.logging.verbose = true;
        // Ensure even if debug is false, verbose wins
        config.logging.levels.debug = false;
        assert_eq!(determine_log_level(&config), "debug");

        // 2. Verbose false, Debug true -> debug
        config.logging.verbose = false;
        config.logging.levels.debug = true;
        assert_eq!(determine_log_level(&config), "debug");

        // 3. Info
        config.logging.levels.debug = false;
        config.logging.levels.info = true;
        assert_eq!(determine_log_level(&config), "info");

        // 4. Warning
        config.logging.levels.info = false;
        config.logging.levels.warning = true;
        assert_eq!(determine_log_level(&config), "warn");

        // 5. Error
        config.logging.levels.warning = false;
        config.logging.levels.error = true;
        assert_eq!(determine_log_level(&config), "error");

        // 6. Critical (mapped to error for now)
        config.logging.levels.error = false;
        config.logging.levels.critical = true;
        assert_eq!(determine_log_level(&config), "error");

        // 7. Off
        config.logging.levels.critical = false;
        assert_eq!(determine_log_level(&config), "off");
    }
}
