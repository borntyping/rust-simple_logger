//! A logger that prints all messages with a readable output format.

extern crate log;
extern crate time;

use log::{Log,Level,Metadata,Record,SetLoggerError};

struct SimpleLogger {
    level: Level,
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} {:<5} [{}] {}",
                time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
                record.level().to_string(),
                record.module_path().unwrap_or_default(),
                record.args());
        }
    }

    fn flush(&self) {
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
/// simple_logger::init_with_level(log::Level::Warn).unwrap();
///
/// warn!("This is an example message.");
/// info!("This message will not be logged.");
/// # }
/// ```
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    static ERROR_LOGGER: SimpleLogger = SimpleLogger { level: Level::Error };
    static WARN_LOGGER: SimpleLogger = SimpleLogger { level: Level::Warn };
    static INFO_LOGGER: SimpleLogger = SimpleLogger { level: Level::Info };
    static DEBUG_LOGGER: SimpleLogger = SimpleLogger { level: Level::Debug };
    static TRACE_LOGGER: SimpleLogger = SimpleLogger { level: Level::Trace };

    log::set_logger(match level {
        Level::Error => &ERROR_LOGGER,
        Level::Warn => &WARN_LOGGER,
        Level::Info => &INFO_LOGGER,
        Level::Debug => &DEBUG_LOGGER,
        Level::Trace => &TRACE_LOGGER,
    })?;
    log::set_max_level(level.to_level_filter());
    Ok(())
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
    init_with_level(Level::Trace)
}
