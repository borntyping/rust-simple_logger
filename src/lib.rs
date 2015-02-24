extern crate log;
extern crate time;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, level: log::LogLevel, _module: &str) -> bool {
        level <= log::LogLevel::Trace
    }

    fn log(&self, record: &log::LogRecord) {
        println!(
            "{} {} [{}] {}",
            time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
            record.level(),
            record.location().module_path,
            record.args());
    }
}

/// Initializes the global logger with a SimpleLogger instance
pub fn init() {
    log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Trace);
        return Box::new(SimpleLogger);
    }).unwrap();
}
