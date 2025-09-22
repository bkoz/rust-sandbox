mod mylib;

use log::{info, debug, error};

fn main() {
    //
    // The logger reads the RUST_LOG env var automatically. Set it to info
    // error, debug, trace as needed.
    // 
    env_logger::init();
    info!("This is an info message");
    debug!("Hello, debug {} !", mylib::add(5, 10));
    error!("ERROR: Hello, world {} !", mylib::add(5, 10));
}
