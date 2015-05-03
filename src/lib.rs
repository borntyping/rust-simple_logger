//! A logger that prints all messages with a readable output format.

extern crate log;
extern crate time;

use log::{Log,LogLevel,LogMetadata,LogRecord,SetLoggerError};

struct SimpleLogger {
    log_level: LogLevel,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!(
                "{} {:<5} [{}] {}",
                time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
                record.level().to_string(),
                record.location().module_path(),
                record.args());
        }
    }
}

/// Initializes the global logger with a SimpleLogger instance with `max_log_level` set to
/// the specified log level.
pub fn init_with_level(log_level: LogLevel) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        return Box::new(SimpleLogger { log_level: log_level });
    })
}

/// Initializes the global logger with a SimpleLogger instance with `max_log_level` set to
/// `LogLevel::Trace`.
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(LogLevel::Trace)
}
