use env_logger;
use log::{error, info, trace};
use networking::server::start_server;
use std::env;
use std::net::Ipv4Addr;

fn main() {
    // Initialize the logger
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    trace!("Number of arguments: {}, arguments: {:?}", args.len(), args);

    // Set default IP address and port
    let mut ip = Ipv4Addr::LOCALHOST;
    let mut port = 11111;
    trace!("Default socket addr: {}:{}", ip, port);

    // Update port and IP address based on provided arguments
    if args.len() > 1 {
        port = args[1].parse::<u16>().expect("Error parsing port");
    } else {
        trace!("Using default port number")
    }
    if args.len() > 2 {
        ip = args[2]
            .parse::<Ipv4Addr>()
            .expect("Error parsing ip address");
    } else {
        trace!("Using default ipv4 address");
    }

    // Start the client
    match start_server(Some(ip), Some(port)) {
        Ok(_) => {
            info!("Server execution finished without error")
        }
        Err(e) => {
            error!("Server execution finished error: {e}")
        }
    }
}
