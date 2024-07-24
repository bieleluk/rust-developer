use crate::common::MessageType;
use log::{info, trace};
use std::error::Error;
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

/// Starts the client, connecting to the specified IP and port or defaults if none are provided.
pub fn start_client(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<(), Box<dyn Error>> {
    // Create the client stream
    let stream = create_client(ip, port)?;
    // Start the client loop to handle communication with the server
    client_loop(stream)?;
    Ok(())
}

/// Creates a TcpStream to connect to the specified IP and port or defaults if none are provided.
pub fn create_client(ip: Option<Ipv4Addr>, port: Option<u16>) -> Result<TcpStream, Box<dyn Error>> {
    // Use the provided IP and port or default to localhost and port 11111
    let ip = ip.unwrap_or(Ipv4Addr::LOCALHOST);
    let port = port.unwrap_or(11111);

    // Create the socket address
    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    trace!("Connecting..."); // Trace log for connection attempt
    let stream = TcpStream::connect(sock_addr)?; // Connect to the server
    trace!("Local address: {}", stream.local_addr().unwrap()); // Trace log for local address
    info!("Connected to {sock_addr}"); // Info log for successful connection
    Ok(stream)
}

/// Main loop to handle communication with the server.
fn client_loop(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    info!(
        "Use one of the following requests:
    .image <image.png>
    .file <file>
    .quit
Any other will be returned as a plain text"
    );

    loop {
        // Read user input from stdin
        info!("Insert the request");
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        // Send the request to the server
        let request = input.trim().as_bytes();
        trace!("Sending request: {:?}", request);
        stream.write_all(request)?;
        trace!("Waiting for the response");

        // Receive the response from the server
        let response = receive_response(&mut stream)?;
        // Take action based on the response
        match response {
            MessageType::Text(text) => {
                info!("Received text: {text}");
            }
            MessageType::Image(_) => {
                info!("Received image...");
                response.to_image()?;
            }
            MessageType::File {
                ref name,
                content: _,
            } => {
                info!("Received file {name}");
                response.to_file()?;
            }
            MessageType::Quit => {
                info!("Quitting");
                return Ok(());
            }
        }
    }
}

/// Receives a response from the server.
fn receive_response(stream: &mut TcpStream) -> Result<MessageType, Box<dyn Error>> {
    let mut buffer = Vec::new();

    loop {
        // Read data from the stream into a temporary buffer
        let mut temp_buffer = [0; 100];
        let bytes_read = match stream.read(&mut temp_buffer) {
            Ok(0) => return Err("Connection closed".into()),
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
                trace!("Failed to deserialize message, continue reading");
            }
        }
    }
}
