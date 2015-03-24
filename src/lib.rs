//! A logger that prints all messages with a readable output format.

extern crate log;
extern crate time;

use log::{Log,LogLevel,LogLevelFilter,LogMetadata,LogRecord,SetLoggerError};

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
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

/// Initializes the global logger with a SimpleLogger instance
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        return Box::new(SimpleLogger);
    })
}
