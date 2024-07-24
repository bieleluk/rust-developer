use crate::common::MessageType;
use anyhow::{Context, Result};
use log::{info, trace};
use std::io::{stdin, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

/// Starts the client, connecting to the specified IP and port.
pub fn start_client(ip: Ipv4Addr, port: u16) -> Result<()> {
    // Create the client stream
    let stream = create_client(ip, port).context("Failed to create client")?;
    // Start the client loop to handle communication with the server
    client_loop(stream).context("Client loop crashed")?;
    Ok(())
}

/// Creates a TcpStream to connect to the specified IP and port.
pub fn create_client(ip: Ipv4Addr, port: u16) -> Result<TcpStream> {
    // Use the provided IP and port to create the socket address
    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    trace!("Connecting..."); // Trace log for connection attempt
    let stream = TcpStream::connect(sock_addr).context("Failed to connect to server")?; // Connect to the server
    trace!("Local address: {}", stream.local_addr().unwrap()); // Trace log for local address
    info!("Connected to {sock_addr}"); // Info log for successful connection
    Ok(stream)
}

/// Main loop to handle communication with the server.
fn client_loop(mut stream: TcpStream) -> Result<()> {
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
        stdin().read_line(&mut input).context("Failed to read a line from stdin")?;

        // Send the request to the server
        let request = input.trim().as_bytes();
        trace!("Sending request: {:?}", request);
        stream.write_all(request).context("Requset sending failed")?;
        trace!("Waiting for the response");

        // Receive the response from the server
        let response = MessageType::receive(&mut stream).context("Response receiving failed")?;
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
