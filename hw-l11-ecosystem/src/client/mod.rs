
use std::error::Error;
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use crate::common::MessageType;

pub fn start_client(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<(), Box<dyn Error>> {
    let stream = create_client(ip, port)?;
    client_loop(stream)?;
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

        // Send request to server
        let request = input.trim().as_bytes();
        stream.write_all(request)?;

        let response = receive_response(&mut stream)?;
        match response {
            MessageType::Text(text) => {
                println!("Received text: {text}");
            }
            MessageType::Image(_) => {
                println!("Received image...");
                response.to_image()?;
            }
            MessageType::File { ref name, content: _ } => {
                println!("Received file {name}");
                response.to_file()?;
            }
            MessageType::Quit => {
                println!("Quitting");
                return Ok(());
            }
        }
    }
}

fn receive_response(stream: &mut TcpStream) -> Result<MessageType, Box<dyn Error>> {
    let mut buffer = Vec::new();

    loop {
        // Read data from the stream into a temporary buffer
        let mut temp_buffer = [0; 100];
        let bytes_read = match stream.read(&mut temp_buffer) {
            Ok(0) => return Err("Connection closed".into()), // Connection closed
            Ok(n) => n,
            Err(e) => return Err(e.into()),
        };

        // Append the read data to the main buffer
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        // Attempt to deserialize the buffer
        match bincode::deserialize::<MessageType>(&buffer) {
            Ok(message) => {
                // Return the successfully deserialized message
                return Ok(message);
            }
            Err(_) => {
                // If deserialization fails, it might be due to incomplete data,
                // so continue reading more data from the stream
                // println!("Failed to deserialize message, continue reading");
            }
        }
    }
}
