use chrono::Local;
use image::{load_from_memory, ImageFormat};
use log::trace;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::{create_dir_all, read, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File { name: String, content: Vec<u8> },
    Quit,
}

/// Custom error type for the crate.
#[derive(Error, Debug)]
pub enum LibError {
    #[error("Wrong message type")]
    WrongMessageType,
    #[error("Image error: {0}")]
    ImageSavingError(#[from] image::ImageError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("I/O error: Connection closed")]
    ConnectionClosed,
    #[error("Address parsing error: {0}")]
    AddressParsingError(#[from] std::net::AddrParseError),
    #[error("Port parsing error: {0}")]
    PortParsingError(#[from] std::num::ParseIntError),
    #[error("Error reading image: {0}")]
    ImageReadingError(String),
    #[error("Wrong image extension. Only PNG images are supported.")]
    WrongImageExtension,
    #[error("Error reading file: {0}")]
    FileReadingError(String),
    #[error("Error parsing file name")]
    FileNameError,
}

impl MessageType {
    /// Constructs a MessageType::Image from a given image file path.
    pub async fn from_image(image_path: &Path) -> Self {
        // Check for png extension
        if image_path.extension() != Some(OsStr::new("png")) {
            return MessageType::Text(LibError::WrongImageExtension.to_string());
        }
        // Read image content
        match read(image_path).await {
            Ok(content) => MessageType::Image(content),
            Err(_) => MessageType::Text(
                LibError::ImageReadingError(format!("{:?}", image_path)).to_string(),
            ),
        }
    }

    /// Constructs a MessageType::File from a given file path.
    pub async fn from_file(file_path: &Path) -> Self {
        // Read file name
        let name = match file_path.file_name() {
            Some(os_name) => os_name.to_string_lossy().into_owned(),
            None => return MessageType::Text(LibError::FileNameError.to_string()),
        };

        // Read file content
        match read(file_path).await {
            Ok(content) => MessageType::File { name, content },
            Err(_) => MessageType::Text(
                LibError::FileReadingError(format!("{:?}", file_path)).to_string(),
            ),
        }
    }

    /// Constructs a MessageType::Text from a given text string.
    pub fn from_text(text: &str) -> Self {
        MessageType::Text(text.to_string())
    }

    /// Saves an Image message to a file.
    pub async fn to_image(&self) -> Result<(), LibError> {
        if let MessageType::Image(ref content) = *self {
            // Create the images directory if it doesn't exist
            create_dir_all("images").await?;
            // Generate a timestamped file name
            let name = format!(
                "{}.png",
                Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
            );
            // Create a PathBuf for the image path
            let path: PathBuf = PathBuf::from("images").join(name);

            // Create and save the image file
            File::create(&path).await?;

            task::block_in_place(move || {
                // Save the image data to a file synchronously
                let img = load_from_memory(content)?;
                img.save_with_format(path, ImageFormat::Png)?;
                Ok(())
            })
        } else {
            Err(LibError::WrongMessageType)
        }
    }

    /// Saves a File message to a file.
    pub async fn to_file(&self) -> Result<(), LibError> {
        if let MessageType::File {
            ref name,
            ref content,
        } = *self
        {
            // Create the files directory if it doesn't exist
            create_dir_all("files").await?;
            // Create a PathBuf for the file path
            let path: PathBuf = PathBuf::from("files").join(name);

            // Create and write the file contents
            let mut file = File::create(path).await?;
            file.write_all(content).await?;
            Ok(())
        } else {
            Err(LibError::WrongMessageType)
        }
    }

    /// Receives a MessageType from tcp stream.
    pub async fn receive(stream: &mut TcpStream) -> Result<Self, LibError> {
        let mut buffer = Vec::new();

        loop {
            // Read data from the stream into a temporary buffer
            let mut temp_buffer = [0; 100];
            let bytes_read = match stream.read(&mut temp_buffer).await {
                Ok(0) => return Err(LibError::ConnectionClosed),
                Ok(n) => n,
                Err(e) => return Err(LibError::IoError(e)),
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

    /// Sends MessageTpe to tcp stream.
    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), LibError> {
        // Serialize the message
        let encoded: Vec<u8> = bincode::serialize(self)?;
        // Write the serialized message to the stream
        stream.write_all(&encoded).await?;
        Ok(())
    }
}

pub fn parse_addr(args: &[String]) -> Result<(Ipv4Addr, u16), LibError> {
    // Set default IP address and port
    let mut ip = Ipv4Addr::LOCALHOST;
    let mut port = 11111;
    trace!("Default socket addr: {}:{}", ip, port);

    trace!("Number of arguments: {}, arguments: {:?}", args.len(), args);

    // Update port and IP address based on provided arguments
    if args.len() > 0 {
        port = args[0].parse::<u16>()?;
    } else {
        trace!("Using default port number")
    }
    if args.len() > 1 {
        ip = args[1].parse::<Ipv4Addr>()?;
    } else {
        trace!("Using default ipv4 address");
    }
    Ok((ip, port))
}
