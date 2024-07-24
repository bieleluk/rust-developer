use crate::MessageType;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::Path;
use std::thread;

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
                let peer_addr = stream.peer_addr().unwrap();
                println!("Accepted connection from {:?}", peer_addr);

                // Spawn a new thread to handle the client
                thread::spawn(move || match handle_client(stream) {
                    Ok(_) => println!("Client {:?} handled successfully", peer_addr),
                    Err(e) => eprintln!("Error handling client {:?}: {}", peer_addr, e),
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let request = receive_request(&mut stream)?;
        let response = create_response(&request)?;
        send_response(&response, &mut stream)?;
        if let MessageType::Quit = response {
            stream.shutdown(std::net::Shutdown::Both)?;
            return Ok(());
        }
    }
}

fn receive_request(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buffer = [0; 100];
    let n = match stream.read(&mut buffer) {
        Ok(0) => return Err("Connection closed".into()), // Connection closed
        Ok(n) => n,
        Err(e) => return Err(e.into()),
    };

    let request = String::from_utf8(buffer[..n].to_vec())?;
    Ok(request)
}

fn create_response(input: &String) -> Result<MessageType, Box<dyn Error>> {
    // Create a message
    let message = if input.starts_with(".quit") {
        Ok(MessageType::Quit)
    } else if input.starts_with(".file ") {
        let path = Path::new(input.trim_start_matches(".file ").trim());
        MessageType::from_file(path)
    } else if input.starts_with(".image ") {
        let path = Path::new(input.trim_start_matches(".image ").trim());
        MessageType::from_image(path)
    } else {
        MessageType::from_text(input)
    }?;
    Ok(message)
}

fn send_response(message: &MessageType, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let encoded: Vec<u8> = bincode::serialize(message).unwrap();
    stream.write_all(&encoded)?;
    Ok(())
}
