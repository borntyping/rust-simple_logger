use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().with_timestamps(true).init().unwrap();

    log::warn!("This is an example message.");
}
