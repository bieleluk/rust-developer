use crate::common::MessageType;
use anyhow::{Context, Result};
use log::{info, trace};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

/// Starts the client, connecting to the specified IP and port.
pub async fn start_client(ip: Ipv4Addr, port: u16) -> Result<()> {
    // Create the client stream
    let stream = create_client(ip, port)
        .await
        .context("Failed to create client")?;
    // Start the client loop to handle communication with the server
    client_loop(stream).await.context("Client loop crashed")?;
    Ok(())
}

/// Creates a TcpStream to connect to the specified IP and port.
pub async fn create_client(ip: Ipv4Addr, port: u16) -> Result<TcpStream> {
    // Use the provided IP and port to create the socket address
    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    trace!("Connecting..."); // Trace log for connection attempt
    let stream = TcpStream::connect(sock_addr)
        .await
        .context("Failed to connect to server")?; // Connect to the server
    trace!("Local address: {}", stream.local_addr().unwrap()); // Trace log for local address
    info!("Connected to {sock_addr}"); // Info log for successful connection
    Ok(stream)
}

/// Main loop to handle communication with the server.
async fn client_loop(mut stream: TcpStream) -> Result<()> {
    info!(
        "Use one of the following requests:
    .image <image.png>
    .file <file>
    .quit
Any other will be returned as a plain text"
    );

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    loop {
        // Read user input from stdin
        info!("Insert the request");
        let mut input = String::new();
        reader
            .read_line(&mut input)
            .await
            .context("Failed to read a line from stdin")?;

        // Send the request to the server
        let request = input.trim().as_bytes();
        trace!("Sending request: {:?}", request);
        stream
            .write_all(request)
            .await
            .context("Requset sending failed")?;
        trace!("Waiting for the response");

        // Receive the response from the server
        let response = MessageType::receive(&mut stream)
            .await
            .context("Response receiving failed")?;
        // Take action based on the response
        match response {
            MessageType::Text(text) => {
                info!("Received text: {text}");
            }
            MessageType::Image(_) => {
                info!("Received image...");
                response.to_image().await?;
            }
            MessageType::File {
                ref name,
                content: _,
            } => {
                info!("Received file {name}");
                response.to_file().await?;
            }
            MessageType::Quit => {
                info!("Quitting");
                return Ok(());
            }
        }
    }
}
