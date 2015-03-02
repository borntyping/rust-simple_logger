#[macro_use]
extern crate log;
extern crate simple_logger;

#[test]
fn log_message() {
    simple_logger::init();
    warn!("This is an example message.");
}
