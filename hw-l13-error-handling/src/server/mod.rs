use crate::common::MessageType;
use log::{error, info, trace};
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::Path;
use std::thread;

/// Starts the server with the specified IP and port.
pub fn start_server(ip: Ipv4Addr, port: u16) -> Result<(), Box<dyn Error>> {
    // Create the server listener
    let server = create_server(ip, port)?;
    // Start the server loop to handle incoming connections
    server_loop(server)?;
    Ok(())
}

/// Creates a TcpListener bound to the specified IP and port.
fn create_server(ip: Ipv4Addr, port: u16) -> Result<TcpListener, Box<dyn Error>> {
    // Use the provided IP and port
    let sock_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));
    trace!("Binding...");
    // Bind the TcpListener to the socket address
    let listener = TcpListener::bind(sock_addr)?;
    info!("Listener binded to {sock_addr}");
    Ok(listener)
}

/// Main loop to accept and handle incoming client connections.
fn server_loop(listener: TcpListener) -> Result<(), Box<dyn Error>> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                info!("Accepted connection from {:?}", peer_addr);
                // Spawn a new thread to handle each client connection
                thread::spawn(move || match handle_client(stream) {
                    Ok(_) => info!("Client {:?} handled successfully", peer_addr),
                    Err(e) => error!("Error handling client {:?}: {}", peer_addr, e),
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
    Ok(())
}

/// Handles communication with a single client.
fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // Get the client's address
    let peer = stream.peer_addr().unwrap();
    loop {
        // Receive a request from the client
        let request = receive_request(&mut stream)?;
        trace!("Received request '{}' from client {}", request, peer);
        // Create a response based on the request
        let response = create_response(&request)?;
        trace!("Sending response to {peer}");
        // Send the response to the client
        send_response(&response, &mut stream)?;
        // Shutdown the connection if Quit message
        if let MessageType::Quit = response {
            info!("Shutting down connection with {peer}");
            stream.shutdown(std::net::Shutdown::Both)?;
            // End the client handling
            return Ok(());
        }
    }
}

/// Receives a request from the client.
fn receive_request(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buffer = [0; 100];
    let n = match stream.read(&mut buffer) {
        Ok(0) => return Err("Connection closed".into()),
        Ok(n) => n,
        Err(e) => return Err(e.into()),
    };
    // Convert bytes to string
    let request = String::from_utf8(buffer[..n].to_vec())?;
    Ok(request)
}

/// Creates a response based on the client's request.
fn create_response(input: &String) -> Result<MessageType, Box<dyn Error>> {
    // Create a message based on the input command
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

/// Sends the response back to the client.
fn send_response(message: &MessageType, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    // Serialize the message
    let encoded: Vec<u8> = bincode::serialize(message).unwrap();
    // Write the serialized message to the stream
    stream.write_all(&encoded)?;
    Ok(())
}
