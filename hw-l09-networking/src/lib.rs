use std::error::Error;
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

#[derive(Serialize, Deserialize, Debug)]

enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File { name: String, content: Vec<u8> },
}

pub fn start_server(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<(), Box<dyn Error>> {
    let server = create_server(ip, port)?;
    server_loop(server)?;
    println!("Server execution finished");

    Ok(())
}

fn create_server(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<TcpListener, Box<dyn Error>> {
    // Use the provided values or default to localhost and port 8080
    let ip = ip.unwrap_or(Ipv4Addr::LOCALHOST);
    let port = port.unwrap_or(11111);

    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    let listener = TcpListener::bind(sock_addr)?;
    println!("Listener {sock_addr} created");
    Ok(listener)
}

fn server_loop(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let received = String::from_utf8_lossy(&buffer[..]);
            println!("Received: {}", received);
        }
        Err(e) => {
            eprintln!("Failed to receive data: {}", e);
        }
    }
    Ok(())
}

pub fn start_client(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<(), Box<dyn Error>> {
    let stream = create_client(ip, port)?;
    client_loop(stream)?;
    println!("Client execution finished");

    Ok(())
}

pub fn create_client(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<TcpStream, Box<dyn Error>> {
    // Use the provided values or default to localhost and port 11111
    let ip = ip.unwrap_or(Ipv4Addr::LOCALHOST);
    let port = port.unwrap_or(11111);

    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    let stream = TcpStream::connect(sock_addr)?;
    println!("Client connected to {sock_addr}");
    Ok(stream)
}

fn client_loop(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        // Read user input from stdin
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.starts_with(".quit") {
            println!("Terminating the client.");
            break;
        } else if input.starts_with(".file ") {
            let path = input.trim_start_matches(".file ").trim();
            println!("Sending the image with {} path", path);
            continue;
        } else if input.starts_with(".image ") {
            let path = input.trim_start_matches(".image ").trim();
            println!("Sending the image with {} path", path);
            continue;
        } else {
            println!("Sending standard text message {}", input);
            continue;
        }
    }

    Ok(())
}
