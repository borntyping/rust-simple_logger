//! A logger that prints all messages with a simple, readable output format.
//!
//! Optional features include timestamps and colored output.
//!
//! ```
//! simple_logger::SimpleLogger::new().env().init().unwrap();
//!
//! log::warn!("This is an example message.");
//! ```
//!
//! Some shortcuts are available for common use cases.
//!
//! Just initialize logging without any configuration:
//!
//! ```
//! simple_logger::init().unwrap();
//! ```
//!
//! Set the log level from the `RUST_LOG` environment variable:
//!
//! ```
//! simple_logger::init_with_env().unwrap();
//! ```
//!
//! Hardcode a default log level:
//!
//! ```
//! simple_logger::init_with_level(log::Level::Warn).unwrap();
//! ```

#[cfg(feature = "chrono")]
use chrono::Local;
#[cfg(feature = "colored")]
use colored::*;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::collections::HashMap;

/// Implements [`Log`] and a set of simple builder methods for configuration.
///
/// Use the various "builder" methods on this struct to configure the logger,
/// then call [`init`] to configure the [`log`] crate.
pub struct SimpleLogger {
    /// The default logging level
    default_level: LevelFilter,

    /// The specific logging level for each module
    ///
    /// This is used to override the default value for some specific modules.
    /// After initialization, the vector is sorted so that the first (prefix) match
    /// directly gives us the desired log level.
    module_levels: Vec<(String, LevelFilter)>,

    /// Whether to include timestamps or not
    ///
    /// This field is only available if the `chrono` feature is enabled.
    #[cfg(feature = "chrono")]
    timestamps: bool,

    /// Whether to use color output or not.
    ///
    /// This field is only available if the `color` feature is enabled.
    #[cfg(feature = "colored")]
    colors: bool,
}

impl SimpleLogger {
    /// Initializes the global logger with a SimpleLogger instance with
    /// default log level set to `Level::Trace`.
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::new().env().init().unwrap();
    /// log::warn!("This is an example message.");
    /// ```
    ///
    /// [`init`]: #method.init
    #[must_use = "You must call init() to begin logging"]
    pub fn new() -> SimpleLogger {
        SimpleLogger {
            default_level: LevelFilter::Trace,
            module_levels: Vec::new(),

            #[cfg(feature = "chrono")]
            timestamps: true,

            #[cfg(feature = "colored")]
            colors: true,
        }
    }

    /// Simulates env_logger behavior, which enables the user to choose log level by
    /// setting a `RUST_LOG` environment variable. The `RUST_LOG` is not set or its value is not
    /// recognized as one of the log levels, this function will use the `Error` level by default.
    ///
    /// You may use the various builder-style methods on this type to configure
    /// the logger, and you must call [`init`] in order to start logging messages.
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// SimpleLogger::from_env().init().unwrap();
    /// log::warn!("This is an example message.");
    /// ```
    ///
    /// [`init`]: #method.init
    #[must_use = "You must call init() to begin logging"]
    #[deprecated(
        since = "1.12.0",
        note = "Use [`env`](#method.env) instead. Will be removed in version 2.0.0."
    )]
    pub fn from_env() -> SimpleLogger {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Error)
            .env()
    }

    /// Simulates env_logger behavior, which enables the user to choose log
    /// level by setting a `RUST_LOG` environment variable. This will use
    /// the default level set by [`with_level`] if `RUST_LOG` is not set or
    /// can't be parsed as a standard log level.
    ///
    /// [`with_level`]: #method.with_level
    #[must_use = "You must call init() to begin logging"]
    pub fn env(mut self) -> SimpleLogger {
        if let Ok(level) = std::env::var("RUST_LOG") {
            match level.to_lowercase().as_str() {
                "trace" => self.default_level = log::LevelFilter::Trace,
                "debug" => self.default_level = log::LevelFilter::Debug,
                "info" => self.default_level = log::LevelFilter::Info,
                "warn" => self.default_level = log::LevelFilter::Warn,
                "error" => self.default_level = log::LevelFilter::Error,
                _ => (),
            }
        };
        self
    }

    /// Set the 'default' log level.
    ///
    /// You can override the default level for specific modules and their sub-modules using [`with_module_level`]
    ///
    /// [`with_module_level`]: #method.with_module_level
    #[must_use = "You must call init() to begin logging"]
    pub fn with_level(mut self, level: LevelFilter) -> SimpleLogger {
        self.default_level = level;
        self
    }

    /// Override the log level for some specific modules.
    ///
    /// This sets the log level of a specific module and all its sub-modules.
    /// When both the level for a parent module as well as a child module are set,
    /// the more specific value is taken. If the log level for the same module is
    /// specified twice, the resulting log level is implementation defined.
    ///
    /// # Examples
    ///
    /// Silence an overly verbose crate:
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// use log::LevelFilter;
    ///
    /// SimpleLogger::new().with_module_level("chatty_dependency", LevelFilter::Warn).init().unwrap();
    /// ```
    ///
    /// Disable logging for all dependencies:
    ///
    /// ```no_run
    /// use simple_logger::SimpleLogger;
    /// use log::LevelFilter;
    ///
    /// SimpleLogger::new()
    ///     .with_level(LevelFilter::Off)
    ///     .with_module_level("my_crate", LevelFilter::Info)
    ///     .init()
    ///     .unwrap();
    /// ```
    #[must_use = "You must call init() to begin logging"]
    pub fn with_module_level(mut self, target: &str, level: LevelFilter) -> SimpleLogger {
        self.module_levels.push((target.to_string(), level));

        /* Normally this is only called in `init` to avoid redundancy, but we can't initialize the logger in tests */
        #[cfg(test)]
        self.module_levels
            .sort_by_key(|(name, _level)| name.len().wrapping_neg());

        self
    }

    /// Override the log level for specific targets.
    #[must_use = "You must call init() to begin logging"]
    #[deprecated(
        since = "1.11.0",
        note = "Use [`with_module_level`](#method.with_module_level) instead. Will be removed in version 2.0.0."
    )]
    pub fn with_target_levels(
        mut self,
        target_levels: HashMap<String, LevelFilter>,
    ) -> SimpleLogger {
        self.module_levels = target_levels.into_iter().collect();

        /* Normally this is only called in `init` to avoid redundancy, but we can't initialize the logger in tests */
        #[cfg(test)]
        self.module_levels
            .sort_by_key(|(name, _level)| name.len().wrapping_neg());

        self
    }

    /// Control whether timestamps are printed or not.
    ///
    /// This method is only available if the `chrono` feature is enabled.
    #[must_use = "You must call init() to begin logging"]
    #[cfg(feature = "chrono")]
    pub fn with_timestamps(mut self, timestamps: bool) -> SimpleLogger {
        self.timestamps = timestamps;
        self
    }

    /// Control whether messages are colored or not.
    ///
    /// This method is only available if the `colored` feature is enabled.
    #[must_use = "You must call init() to begin logging"]
    #[cfg(feature = "colored")]
    pub fn with_colors(mut self, colors: bool) -> SimpleLogger {
        self.colors = colors;
        self
    }

    /// 'Init' the actual logger, instantiate it and configure it,
    /// this method MUST be called in order for the logger to be effective.
    pub fn init(mut self) -> Result<(), SetLoggerError> {
        #[cfg(all(windows, feature = "colored"))]
        set_up_color_terminal();

        /* Sort all module levels from most specific to least specific. The length of the module
         * name is used instead of its actual depth to avoid module name parsing.
         */
        self.module_levels
            .sort_by_key(|(name, _level)| name.len().wrapping_neg());
        let max_level = self
            .module_levels
            .iter()
            .map(|(_name, level)| level)
            .copied()
            .max();
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
        &metadata.level().to_level_filter()
            <= self
                .module_levels
                .iter()
                /* At this point the Vec is already sorted so that we can simply take
                 * the first match
                 */
                .find(|(name, _level)| metadata.target().starts_with(name))
                .map(|(_name, level)| level)
                .unwrap_or(&self.default_level)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = {
                #[cfg(feature = "colored")]
                {
                    if self.colors {
                        match record.level() {
                            Level::Error => record.level().to_string().red().to_string(),
                            Level::Warn => record.level().to_string().yellow().to_string(),
                            Level::Info => record.level().to_string().cyan().to_string(),
                            Level::Debug => record.level().to_string().purple().to_string(),
                            Level::Trace => record.level().to_string().normal().to_string(),
                        }
                    } else {
                        record.level().to_string()
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
            if self.timestamps {
                #[cfg(not(feature = "stderr"))]
                println!(
                    "{} {:<5} [{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                    level_string,
                    target,
                    record.args()
                );
                #[cfg(feature = "stderr")]
                eprintln!(
                    "{} {:<5} [{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                    level_string,
                    target,
                    record.args()
                );
                return;
            }

            #[cfg(not(feature = "stderr"))]
            println!("{:<5} [{}] {}", level_string, target, record.args());
            #[cfg(feature = "stderr")]
            eprintln!("{:<5} [{}] {}", level_string, target, record.args());
        }
    }

    fn flush(&self) {}
}

#[cfg(all(windows, feature = "colored"))]
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

/// Initialise the logger with it's default configuration.
///
/// Log messages will not be filtered.
/// The `RUST_LOG` environment variable is not used.
pub fn init() -> Result<(), SetLoggerError> {
    SimpleLogger::new().init()
}

/// Initialise the logger with the `RUST_LOG` environment variable.
///
/// Log messages will be filtered based on the `RUST_LOG` environment variable.
pub fn init_with_env() -> Result<(), SetLoggerError> {
    SimpleLogger::new().env().init()
}

/// Initialise the logger with a specific log level.
///
/// Log messages below the given [`Level`] will be filtered.
/// The `RUST_LOG` environment variable is not used.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    SimpleLogger::new()
        .with_level(level.to_level_filter())
        .init()
}

/// Use [`init_with_env`] instead.
///
/// This does the same as [`init_with_env`] but unwraps the result.
#[deprecated(
    since = "1.12.0",
    note = "Use [`init_with_env`] instead, which does not unwrap the result. Will be removed in version 2.0.0."
)]
pub fn init_by_env() {
    init_with_env().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_module_levels_allowlist() {
        let logger = SimpleLogger::new()
            .with_level(LevelFilter::Off)
            .with_module_level("my_crate", LevelFilter::Info);

        assert!(logger.enabled(&create_log("my_crate", Level::Info)));
        assert!(logger.enabled(&create_log("my_crate::module", Level::Info)));
        assert!(!logger.enabled(&create_log("my_crate::module", Level::Debug)));
        assert!(!logger.enabled(&create_log("not_my_crate", Level::Debug)));
        assert!(!logger.enabled(&create_log("not_my_crate::module", Level::Error)));
    }

    #[test]
    fn test_module_levels_denylist() {
        let logger = SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .with_module_level("my_crate", LevelFilter::Trace)
            .with_module_level("chatty_dependency", LevelFilter::Info);

        assert!(logger.enabled(&create_log("my_crate", Level::Info)));
        assert!(logger.enabled(&create_log("my_crate", Level::Trace)));
        assert!(logger.enabled(&create_log("my_crate::module", Level::Info)));
        assert!(logger.enabled(&create_log("my_crate::module", Level::Trace)));
        assert!(logger.enabled(&create_log("not_my_crate", Level::Debug)));
        assert!(!logger.enabled(&create_log("not_my_crate::module", Level::Trace)));
        assert!(logger.enabled(&create_log("chatty_dependency", Level::Info)));
        assert!(!logger.enabled(&create_log("chatty_dependency", Level::Debug)));
        assert!(!logger.enabled(&create_log("chatty_dependency::module", Level::Debug)));
        assert!(logger.enabled(&create_log("chatty_dependency::module", Level::Warn)));
    }

    #[test]
    #[cfg(feature = "chrono")]
    fn test_with_timestamps() {
        let mut builder = SimpleLogger::new();
        assert!(builder.timestamps == true);

        builder = builder.with_timestamps(false);
        assert!(builder.timestamps == false);
    }

    #[test]
    #[cfg(feature = "colored")]
    fn test_with_colors() {
        let mut builder = SimpleLogger::new();
        assert!(builder.colors == true);

        builder = builder.with_colors(false);
        assert!(builder.colors == false);
    }

    fn create_log(name: &str, level: Level) -> Metadata {
        let mut builder = Metadata::builder();
        builder.level(level);
        builder.target(name);
        builder.build()
    }
}
