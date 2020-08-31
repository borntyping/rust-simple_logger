//! A logger that prints all messages with a readable output format.

#[cfg(feature = "chrono")]
use chrono::Local;
#[cfg(feature = "colored")]
use colored::*;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::collections::HashMap;

pub struct SimpleLogger {
    /// The default logging level
    default_level: LevelFilter,
    /// The specific logging level for each modules
    module_levels: HashMap<String, LevelFilter>,
}

impl SimpleLogger {
    /// Initializes the global logger with a SimpleLogger instance with
    /// default log level set to `Level::Trace`.
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::new();
    /// log::warn!("This is an example message.");
    /// ```
    pub fn new() -> SimpleLogger {
        SimpleLogger {
            default_level: LevelFilter::Trace,
            module_levels: HashMap::new(),
        }
    }

    /// A macro for simulating env_logger behavior, which enables the user to choose log level by
    /// setting a `RUST_LOG` environment variable. The `RUST_LOG` is not set or its value is not
    /// recognized as one of the log levels, this function with use the `Error` level by default.
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::from_env();
    /// log::warn!("This is an example message.");
    /// ```
    pub fn from_env() -> SimpleLogger {
        let level = match std::env::var("RUST_LOG") {
            Ok(x) => match x.to_lowercase().as_str() {
                "trace" => log::LevelFilter::Trace,
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "warn" => log::LevelFilter::Warn,
                _ => log::LevelFilter::Error,
            },
            _ => log::LevelFilter::Error,
        };

        SimpleLogger::new().with_level(level)
    }

    /// Set the 'default' log level.
    pub fn with_level(mut self, level: LevelFilter) -> SimpleLogger {
        self.default_level = level;
        self
    }

    /// Override the log level for specific module.
    ///
    /// # Examples
    ///
    /// Change log level for specific crate:
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// use log::LevelFilter;
    ///
    /// SimpleLogger::new().with_module_level("something", LevelFilter::Warn).init();
    /// ```
    ///
    /// Disable logging for specific crate:
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// use log::LevelFilter;
    ///
    /// SimpleLogger::new().with_module_level("something", LevelFilter::Off).init();
    /// ```
    pub fn with_module_level(mut self, target: &str, level: LevelFilter) -> SimpleLogger {
        self.module_levels.insert(target.to_string(), level);
        self
    }

    /// Override the log level for specific targets.
    pub fn with_target_levels(
        mut self,
        target_levels: HashMap<String, LevelFilter>,
    ) -> SimpleLogger {
        self.module_levels = target_levels;
        self
    }

    /// 'Init' the actual logger, instantiate it and configure it,
    /// this method MUST be called in order for the logger to be effective.
    pub fn init(self) -> Result<(), SetLoggerError> {
        #[cfg(all(windows, feature = "colored"))]
        set_up_color_terminal();

        let max_level = self.module_levels.values().copied().max();
        let max_level = max_level
            .map(|lvl| lvl.max(self.default_level))
            .unwrap_or(self.default_level);
        log::set_max_level(max_level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }
}

impl Default for SimpleLogger {
    /// See [this](struct.SimpleLogger.html#method.new)
    fn default() -> Self {
        SimpleLogger::new()
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level().to_level_filter()
            <= self
                .module_levels
                .get(metadata.target())
                .copied()
                .unwrap_or_else(|| self.default_level)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = {
                #[cfg(feature = "colored")]
                {
                    match record.level() {
                        Level::Error => record.level().to_string().red(),
                        Level::Warn => record.level().to_string().yellow(),
                        Level::Info => record.level().to_string().cyan(),
                        Level::Debug => record.level().to_string().purple(),
                        Level::Trace => record.level().to_string().normal(),
                    }
                }
                #[cfg(not(feature = "colored"))]
                {
                    record.level().to_string()
                }
            };
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };
            #[cfg(feature = "chrono")]
            {
                println!(
                    "{} {:<5} [{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                    level_string,
                    target,
                    record.args()
                );
            }
            #[cfg(not(feature = "chrono"))]
            {
                println!("{:<5} [{}] {}", level_string, target, record.args());
            }
        }
    }

    fn flush(&self) {}
}

#[cfg(windows)]
fn set_up_color_terminal() {
    use atty::Stream;

    if atty::is(Stream::Stdout) {
        unsafe {
            use winapi::um::consoleapi::*;
            use winapi::um::handleapi::*;
            use winapi::um::processenv::*;
            use winapi::um::winbase::*;
            use winapi::um::wincon::*;

            let stdout = GetStdHandle(STD_OUTPUT_HANDLE);

            if stdout == INVALID_HANDLE_VALUE {
                return;
            }

            let mut mode: winapi::shared::minwindef::DWORD = 0;

            if GetConsoleMode(stdout, &mut mode) == 0 {
                return;
            }

            SetConsoleMode(stdout, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

/// See [this](struct.SimpleLogger.html#method.with_level)
#[deprecated(since = "1.8.0", note = "Please use the Builder pattern instead.")]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    SimpleLogger::new()
        .with_level(level.to_level_filter())
        .init()
}

/// See [this](struct.SimpleLogger.html#method.new)
#[deprecated(since = "1.8.0", note = "Please use the Builder pattern instead.")]
pub fn init() -> Result<(), SetLoggerError> {
    SimpleLogger::new().init()
}

/// See [this](struct.SimpleLogger.html#method.from_env)
#[deprecated(since = "1.8.0", note = "Please use the Builder pattern instead.")]
pub fn init_by_env() {
    SimpleLogger::from_env().init().unwrap()
}
