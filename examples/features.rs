use simple_logger::SimpleLogger;

fn main() {
    let mut builder = SimpleLogger::new();

    #[cfg(feature = "chrono")]
    builder = builder.with_timestamps(true);

    builder.init().unwrap();

    log::warn!("This is an example message.");
}
