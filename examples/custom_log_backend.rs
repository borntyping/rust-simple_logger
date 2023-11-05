use simple_logger::{LogBackend, SimpleLogger};

struct CustomLogBackend;

impl LogBackend for CustomLogBackend {
    fn log(&self, message: String) {
        println!("[Custom log backend] {}", message);
    }
}

impl CustomLogBackend {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

fn main() {
    SimpleLogger::new()
        .with_backend(CustomLogBackend::new())
        .init()
        .unwrap();

    log::warn!("This is an example message.");
}
