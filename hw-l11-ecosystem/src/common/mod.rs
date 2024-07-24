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
    pub fn from_image(image_path: &Path) -> Result<Self, Box<dyn Error>> {
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

    pub fn from_file(file_path: &Path) -> Result<Self, Box<dyn Error>> {
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

    pub fn from_text(text: &str) -> Result<Self, Box<dyn Error>> {
        Ok(MessageType::Text(text.to_string()))
    }

    pub fn to_image(&self) -> Result<(), Box<dyn Error>> {
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

    pub fn to_file(&self) -> Result<(), Box<dyn Error>> {
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
