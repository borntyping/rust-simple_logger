use simple_logger::SimpleLogger;

fn main() {
    let mut builder = SimpleLogger::new();

    #[cfg(feature = "chrono")]
    {
        builder = builder.with_timestamps(false);
    }

    #[cfg(feature = "colored")]
    {
        builder = builder.with_colors(false);
    }

    builder.init().unwrap();

    log::warn!("This is an example message.");
}
