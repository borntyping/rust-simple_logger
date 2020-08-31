use log::Level;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_level(Level::Warn).init().unwrap();

    log::warn!("This will be logged.");
    log::info!("This will NOT be logged.");
}
