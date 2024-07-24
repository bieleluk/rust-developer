use networking::start_server;
use std::env;
use std::net::Ipv4Addr;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Set default IP address and port
    let mut ip = Ipv4Addr::LOCALHOST;
    let mut port = 11111;

    println!("{}, {:?}", args.len(), args);

    // Update port and IP address based on provided arguments
    if args.len() > 1 {
        port = args[1].parse::<u16>().expect("Error parsing port");
    } else {
        println!("Using default port number")
    }
    if args.len() > 2 {
        ip = args[2]
            .parse::<Ipv4Addr>()
            .expect("Error parsing ip address");
    } else {
        println!("Using default ipv4 address");
    }

    // Start the client
    match start_server(Some(ip), Some(port)) {
        Ok(_) => {
            println!("Server execution finished without error")
        }
        Err(e) => {
            eprintln!("Server execution finished error: {e}")
        }
    }
}
