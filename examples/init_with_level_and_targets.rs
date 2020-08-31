use log::Level;

fn main() {
    simple_logger::init_with_level_and_targets(Level::Info, &["wrong_target"]).unwrap();

    log::info!("This will NOT be logged. (Wrong target)");
}
