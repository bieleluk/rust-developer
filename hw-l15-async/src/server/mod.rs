use crate::common::MessageType;
use anyhow::{bail, Context, Result};
use log::{error, info, trace};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Starts the server with the specified IP and port.
pub async fn start_server(ip: Ipv4Addr, port: u16) -> Result<()> {
    // Create the server listener
    let server = create_server(ip, port)
        .await
        .context("Failed to create server")?;
    // Start the server loop to handle incoming connections
    server_loop(server).await.context("Server loop crashed")?;
    Ok(())
}

/// Creates a TcpListener bound to the specified IP and port.
async fn create_server(ip: Ipv4Addr, port: u16) -> Result<TcpListener> {
    // Use the provided IP and port
    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    trace!("Binding...");
    // Bind the TcpListener to the socket address
    let listener = TcpListener::bind(sock_addr)
        .await
        .context("Failed to bind TCP listener")?;
    info!("Listener binded to {sock_addr}");
    Ok(listener)
}

/// Main loop to accept and handle incoming client connections.
async fn server_loop(listener: TcpListener) -> Result<()> {
    loop {
        match listener.accept().await {
            Ok((stream, peer_addr)) => {
                info!("Accepted connection from {:?}", peer_addr);

                // Spawn a new task to handle each client connection
                tokio::spawn(async move {
                    match handle_client(stream).await {
                        Ok(_) => info!("Client {:?} handled successfully", peer_addr),
                        Err(e) => error!("Error handling client {:?}: {}", peer_addr, e),
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

/// Handles communication with a single client.
async fn handle_client(mut stream: TcpStream) -> Result<()> {
    // Get the client's address
    let peer = stream.peer_addr().unwrap();
    loop {
        // Receive a request from the client
        let request = receive_request(&mut stream).await.context("Request receiving failed")?;
        trace!("Received request '{}' from client {}", request, peer);
        // Create a response based on the request
        let response = create_response(&request).await.context("Failed to create response")?;
        trace!("Sending response to {peer}");
        // Send the response to the client
        response
            .send(&mut stream).await.context("Response sending failed")?;
        // Shutdown the connection if Quit message
        if let MessageType::Quit = response {
            info!("Shutting down connection with {peer}");
            stream.shutdown().await.context("Failed to terminate connection")?;
            // End the client handling
            return Ok(());
        }
    }
}

/// Receives a request from the client.
async fn receive_request(stream: &mut TcpStream) -> Result<String> {
    let mut buffer = [0; 100];
    let n = match stream.read(&mut buffer).await {
        Ok(0) => bail!("Connection closed"),
        Ok(n) => n,
        Err(e) => bail!(e),
    };
    // Convert bytes to string
    let request =
        String::from_utf8(buffer[..n].to_vec()).context("[u8] to String conversion failed")?;
    Ok(request)
}

/// Creates a response based on the client's request.
async fn create_response(input: &String) -> Result<MessageType> {
    // Create a message based on the input command
    let message = if input.starts_with(".quit") {
        MessageType::Quit
    } else if input.starts_with(".file ") {
        let path = Path::new(input.trim_start_matches(".file ").trim());
        MessageType::from_file(path).await
    } else if input.starts_with(".image ") {
        let path = Path::new(input.trim_start_matches(".image ").trim());
        MessageType::from_image(path).await
    } else {
        MessageType::from_text(input)
    };
    Ok(message)
}
