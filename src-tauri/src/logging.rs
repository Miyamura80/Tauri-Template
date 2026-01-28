use crate::config::get_config;
use regex::Regex;
use std::io;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Layer};

struct RedactingWriter<W> {
    inner: W,
    patterns: Vec<(Regex, String)>,
}

impl<W: io::Write> io::Write for RedactingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        let mut redacted = s.to_string();
        for (re, replacement) in &self.patterns {
            redacted = re.replace_all(&redacted, replacement).to_string();
        }
        self.inner.write_all(redacted.as_bytes())?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

struct RedactingMakeWriter {
    patterns: Vec<(Regex, String)>,
}

impl<'a> fmt::MakeWriter<'a> for RedactingMakeWriter {
    type Writer = RedactingWriter<io::Stdout>;

    fn make_writer(&self) -> Self::Writer {
        RedactingWriter {
            inner: io::stdout(),
            patterns: self.patterns.clone(),
        }
    }
}

pub fn init_logging() {
    let config = get_config();

    // Determine the log level from config. pick the most verbose one enabled.
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

    // Base formatter configuration
    let show_file = config.logging.format.location.show_file;
    let show_line = config.logging.format.location.show_line;

    // Setup redaction patterns
    let mut patterns = Vec::new();
    if config.logging.redaction.enabled {
        for p in &config.logging.redaction.patterns {
            if let Ok(re) = Regex::new(&p.regex) {
                patterns.push((re, p.placeholder.clone()));
            }
        }
    }

    let make_writer = RedactingMakeWriter { patterns };

    // Use Layer::boxed() to unify the types of the if/else branches
    let fmt_layer = if !config.logging.format.show_time {
        fmt::layer()
            .with_writer(make_writer)
            .with_target(show_file)
            .with_file(show_file)
            .with_line_number(show_line)
            .with_thread_ids(false)
            .without_time()
            .boxed()
    } else {
        fmt::layer()
            .with_writer(make_writer)
            .with_target(show_file)
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
