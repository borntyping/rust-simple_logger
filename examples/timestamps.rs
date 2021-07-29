use simple_logger::SimpleLogger;

#[cfg(feature = "chrono")]
fn main() {
    SimpleLogger::new().with_timestamps(true).init().unwrap();

    log::warn!("This is an example message.");
}
