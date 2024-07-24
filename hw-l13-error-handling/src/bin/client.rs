use env_logger;
use log::{error, info};
use networking::client::start_client;
use networking::common::parse_addr;
use std::env;

fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse port and ipv4 addr from the arguments
    let args: Vec<String> = env::args().collect();
    let (ip, port) = parse_addr(&args[1..]).unwrap_or_else(|e| {
        error!("{e}");
        std::process::exit(1);
    });
    info!("Parsed address is: {}:{}", ip, port);

    // Start the client
    match start_client(ip, port) {
        Ok(_) => {
            info!("Client execution finished without error");
        }
        Err(e) => {
            error!("Client execution finished error: {e}");
            std::process::exit(1);
        }
    }
}
