use std::thread;
use networking::{start_server, start_client};
use std::net::Ipv4Addr;

fn main() {
    let ip: Ipv4Addr = Ipv4Addr::new(127,0,0,1);
    let port: u16 = 11111;

    // Start the server in a separate thread
    let server_handle = thread::spawn(move || {
        let _ = start_server(Some(ip), Some(port));
    });

    // Give the server some time to start
    thread::sleep(std::time::Duration::from_secs(1));

    // Start the client
    match start_client(Some(ip), Some(port)) {
        Ok(_) => {
            println!("Client execution finished without error")
        }
        Err(e) => {
            eprintln!("Client execution finished error: {e}")
        }
    }

    // Wait for the server thread to finish (it won't in this example)
    server_handle.join().unwrap();
}