//! A logger that prints all messages with a readable output format.

#[macro_use]
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

/// Initializes the global logger with a SimpleLogger instance with
/// `max_log_level` set to a specific log level.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate simple_logger;
/// #
/// # fn main() {
/// simple_logger::init_with_level(log::LogLevel::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
/// ```
pub fn init_with_level(log_level: LogLevel) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level.to_log_level_filter());
        return Box::new(SimpleLogger { log_level: log_level });
    })
}

/// Initializes the global logger with a SimpleLogger instance with
/// `max_log_level` set to `LogLevel::Trace`.
///
/// ```
/// # #[macro_use] extern crate log;
/// # extern crate simple_logger;
/// #
/// # fn main() {
/// simple_logger::init().unwrap();
/// warn!("This is an example message.");
/// # }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(LogLevel::Trace)
}
