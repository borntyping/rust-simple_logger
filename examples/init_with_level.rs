#[macro_use]
extern crate log;
extern crate simple_logger;

use log::LogLevel;

fn main() {
    simple_logger::init_with_level(LogLevel::Warn).unwrap();

    warn!("This will be logged.");
    info!("This will NOT be logged.");
}

