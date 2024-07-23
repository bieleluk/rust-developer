use chrono::Local;
use image::{load_from_memory, ImageFormat};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File { name: String, content: Vec<u8> },
    Quit,
}

impl MessageType {
    fn from_image(image_path: &Path) -> Result<Self, Box<dyn Error>> {
        if image_path.extension() != Some(OsStr::new("png")) {
            return Ok(MessageType::Text(String::from(
                "Wrong image extension. Only PNG files are supported.",
            )));
        }
        match File::open(image_path) {
            Ok(mut file) => {
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                Ok(MessageType::Image(content))
            }
            Err(_) => Ok(MessageType::Text(String::from("Error reading image"))),
        }
    }

    fn from_file(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                let name = file_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                Ok(MessageType::File { name, content })
            }
            Err(_) => Ok(MessageType::Text(String::from("Error reading file"))),
        }
    }

    fn from_text(text: &str) -> Result<Self, Box<dyn Error>> {
        Ok(MessageType::Text(text.to_string()))
    }

    fn to_image(&self) -> Result<(), Box<dyn Error>> {
        if let MessageType::Image(ref content) = *self {
            // Create images directory
            create_dir_all("images")?;
            // timestamp name
            let name = format!("{}.png", Local::now().format("%Y-%m-%d_%H-%M-%S").to_string());
            // Create a PathBuf for the image path
            let path: PathBuf = PathBuf::from("images").join(name);

            File::create(&path)?;
            let img = load_from_memory(content)?;
            img.save_with_format(path, ImageFormat::Png)?;

            Ok(())
        } else {
            Err("Not an Image message type".into())
        }
    }

    fn to_file(&self) -> Result<(), Box<dyn Error>> {
        if let MessageType::File { ref name, ref content } = *self {
            // Create files directory
            create_dir_all("files")?;
            // Create a PathBuf for the image path
            let path: PathBuf = PathBuf::from("files").join(name);

            let mut file = File::create(path)?;
            file.write_all(content)?;
            Ok(())
        } else {
            Err("Not a File message type".into())
        }
    }
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
            Ok(stream) => match handle_client(stream) {
                Ok(_) => {}
                Err(e) => {
                    println!("Handling of client finished with error: {}", e)
                }
            },
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
