use log::{Level, LevelFilter};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(Level::Info)
        .with_target_level("init_with_target_level", LevelFilter::Off)
        .init()
        .unwrap();

    log::info!("This will NOT be logged. (Target disabled)");
}
