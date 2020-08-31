use log::Level;

fn main() {
    simple_logger::init_with_level(Level::Warn).unwrap();

    log::warn!("This will be logged.");
    log::info!("This will NOT be logged.");
}
