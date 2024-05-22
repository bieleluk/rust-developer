use std::error::Error;
mod str_utils;
use str_utils::{check_transformation, get_data, transform};

pub fn input_parser() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

pub fn data_processor() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

pub fn run_complete(transformation: &str) -> Result<String, Box<dyn Error>> {
    // Read a string from stdin
    let transformation = check_transformation(transformation)?;
    println!("Insert string and press the Enter");
    let input_str = get_data(transformation)?;

    transform(&input_str, transformation)
}
