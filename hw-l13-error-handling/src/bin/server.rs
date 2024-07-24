use env_logger;
use log::{error, info};
use networking::common::parse_addr;
use networking::server::start_server;
use std::env;

fn main() {
    // Initialize the logger
    env_logger::init();

    // Parse port and ipv4 addr from the arguments
    let args: Vec<String> = env::args().collect();
    let (ip, port) = parse_addr(&args[1..]);
    info!("Parsed address is: {}:{}", ip, port);

    // Start the client
    match start_server(ip, port) {
        Ok(_) => {
            info!("Server execution finished without error");
        }
        Err(e) => {
            error!("Server execution finished error: {e}");
            std::process::exit(1);
        }
    }
}
