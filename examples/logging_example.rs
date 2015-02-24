#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init();

    warn!("This is an example message.");
}
