use chrono::Local;
use image::{load_from_memory, ImageFormat};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File { name: String, content: Vec<u8> },
    Quit,
}

impl MessageType {
    /// Constructs a MessageType::Image from a given image file path.
    /// Only PNG files are supported.
    pub fn from_image(image_path: &Path) -> Result<Self, Box<dyn Error>> {
        // Check if the file has a .png extension
        if image_path.extension() != Some(OsStr::new("png")) {
            return Ok(MessageType::Text(String::from(
                "Wrong image extension. Only PNG files are supported.",
            )));
        }
        // Try to open the file and read its contents
        match File::open(image_path) {
            Ok(mut file) => {
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                Ok(MessageType::Image(content))
            }
            Err(_) => Ok(MessageType::Text(String::from("Error reading image"))),
        }
    }

    /// Constructs a MessageType::File from a given file path.
    pub fn from_file(file_path: &Path) -> Result<Self, Box<dyn Error>> {
        // Try to open the file and read its contents
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

    /// Constructs a MessageType::Text from a given text string.
    pub fn from_text(text: &str) -> Result<Self, Box<dyn Error>> {
        Ok(MessageType::Text(text.to_string()))
    }

    /// Saves an Image message to a file.
    pub fn to_image(&self) -> Result<(), Box<dyn Error>> {
        if let MessageType::Image(ref content) = *self {
            // Create the images directory if it doesn't exist
            create_dir_all("images")?;
            // Generate a timestamped file name
            let name = format!(
                "{}.png",
                Local::now().format("%Y-%m-%d_%H-%M-%S").to_string()
            );
            // Create a PathBuf for the image path
            let path: PathBuf = PathBuf::from("images").join(name);

            // Create and save the image file
            File::create(&path)?;
            let img = load_from_memory(content)?;
            img.save_with_format(path, ImageFormat::Png)?;

            Ok(())
        } else {
            Err("Not an Image message type".into())
        }
    }

    /// Saves a File message to a file.
    pub fn to_file(&self) -> Result<(), Box<dyn Error>> {
        if let MessageType::File {
            ref name,
            ref content,
        } = *self
        {
            // Create the files directory if it doesn't exist
            create_dir_all("files")?;
            // Create a PathBuf for the file path
            let path: PathBuf = PathBuf::from("files").join(name);

            // Create and write the file contents
            let mut file = File::create(path)?;
            file.write_all(content)?;
            Ok(())
        } else {
            Err("Not a File message type".into())
        }
    }
}
