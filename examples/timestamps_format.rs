use simple_logger::SimpleLogger;
use time::macros::format_description;

fn main() {
    SimpleLogger::new()
        .env()
        .with_custom_timestamps(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"))
        .init()
        .unwrap();

    log::warn!("This is an example message with custom timestamp format.");
}
