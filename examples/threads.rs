use simple_logger::SimpleLogger;

fn main() {
    #[cfg(feature = "thread_ids")]
    SimpleLogger::new().with_thread_ids(true).init().unwrap();

    #[cfg(not(feature = "thread_ids"))]
    SimpleLogger::new().init().unwrap();

    std::thread::spawn(|| {
        log::warn!("Unnamed thread logs here.");
    })
    .join()
    .unwrap();

    std::thread::Builder::new()
        .name("named_thread".to_string())
        .spawn(|| {
            log::warn!("Named thread logs here.");
        })
        .unwrap()
        .join()
        .unwrap();

    log::warn!("This is an example message.");
}
