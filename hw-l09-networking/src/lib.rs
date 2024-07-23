use std::error::Error;
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
use std::path::Path;

enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File { name: String, content: Vec<u8> },
}

impl MessageType {
    fn new_png_image(image_path: &Path) -> Result<Self, Box<dyn Error>> {
        if image_path.extension() != Some(OsStr::new("png")) {
            return Err("Wrong image extension. Only PNG files are supported.".into());
        }
        let mut file = File::open(image_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(MessageType::Image(content))
    }

    fn new_file(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        let name = file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        Ok(MessageType::File { name, content })
    }

    fn new_text(text: &str) -> Result<Self, Box<dyn Error>> {
        Ok(MessageType::Text(text.to_string()))
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

        // Check for quit command
        if input.starts_with(".quit") {
            println!("Terminating the client.");
            break;
        }

        // Create a message
        let message = if input.starts_with(".file ") {
            let path = Path::new(input.trim_start_matches(".file ").trim());
            MessageType::new_file(path)
        } else if input.starts_with(".image ") {
            let path = Path::new(input.trim_start_matches(".image ").trim());
            MessageType::new_png_image(path)
        } else {
            MessageType::new_text(input)
        }?;
        println!("{:?}", message);
    }

    Ok(())
}
