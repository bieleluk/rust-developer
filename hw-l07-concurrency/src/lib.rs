use std::error::Error;
mod str_utils;
use std::io::{stdin, Read};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};
use str_utils::{
    csv_formatted_str, csv_open, csv_parse, double_str, remove_spaces, reverse, slugify_str,
    to_lowercase, to_uppercase,
};

// Define an enum for the possible transformations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Transformation {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    Csv,
    Double,
    Reverse,
}

#[derive(PartialEq)]
pub enum AppType {
    Threaded,
    Unattended,
}

impl FromStr for Transformation {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Transformation, Self::Err> {
        match input.to_lowercase().as_str() {
            "lowercase" => Ok(Transformation::Lowercase),
            "uppercase" => Ok(Transformation::Uppercase),
            "no-spaces" => Ok(Transformation::NoSpaces),
            "slugify" => Ok(Transformation::Slugify),
            "csv" => Ok(Transformation::Csv),
            "double" => Ok(Transformation::Double),
            "reverse" => Ok(Transformation::Reverse),
            _ => Err("Unknown Transformation error"),
        }
    }
}

pub fn input_parser(tx: Sender<(Transformation, String)>) -> Result<(), Box<dyn Error>> {
    let mut input_str = String::new();

    loop {
        input_str.clear();

        println!("Insert transformation and data: <command> <input>");
        stdin().read_line(&mut input_str)?;

        let parts: Vec<&str> = input_str.trim().splitn(2, ' ').collect();
        if parts.len() == 2 {
            let transformation_str = parts[0];
            let input_str = parts[1];

            match Transformation::from_str(transformation_str) {
                Ok(command) => {
                    let message = (command, input_str.to_string());
                    tx.send(message)?;
                }
                Err(e) => eprintln!("Error in parsing command: {}", e),
            }
        } else {
            eprintln!("Invalid input -- expected: <command> <input>");
        }
    }
}

pub fn data_processor(rx: Receiver<(Transformation, String)>) -> Result<(), Box<dyn Error>> {
    for request in rx {
        let (transformation, input_str) = request;
        match transform(&input_str, transformation, AppType::Threaded) {
            Err(e) => eprintln!("Error '{e}' occurred!"),
            Ok(result) => println!("{result}"),
        };
    }
    Ok(())
}

pub fn run_unattended(transformation_str: &str) -> Result<String, Box<dyn Error>> {
    // Evaluate a valid transformation
    let transformation = Transformation::from_str(transformation_str)?;
    println!("Insert string and press the Enter");
    // Read a string from stdin
    let input_str = get_data(transformation)?;
    // Transform the input and return the result
    transform(&input_str, transformation, AppType::Unattended)
}

pub fn get_data(transformation: Transformation) -> Result<String, Box<dyn Error>> {
    let mut input_str = String::new();

    // Handle CSV case requiring multi-line input
    match transformation {
        Transformation::Csv => stdin().read_to_string(&mut input_str)?,
        _ => stdin().read_line(&mut input_str)?, // valid_transmutation() guarantees no bad inputs
    };
    Ok(input_str)
}

pub fn transform(
    input_str: &str,
    transformation: Transformation,
    app_type: AppType,
) -> Result<String, Box<dyn Error>> {
    // In case csv command in threaded application, the file has to be loaded to string at first
    let input_str = if transformation == Transformation::Csv && app_type == AppType::Threaded {
        csv_open(input_str)?
    } else {
        input_str.to_string()
    };

    match transformation {
        Transformation::Lowercase => to_lowercase(&input_str),
        Transformation::Uppercase => to_uppercase(&input_str),
        Transformation::NoSpaces => remove_spaces(&input_str),
        Transformation::Slugify => slugify_str(&input_str),
        Transformation::Csv => csv_formatted_str(csv_parse(&input_str)?),
        Transformation::Double => double_str(&input_str),
        Transformation::Reverse => reverse(&input_str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase() {
        assert_eq!(
            transform(
                "Hello, World!",
                Transformation::Lowercase,
                AppType::Unattended
            )
            .unwrap(),
            "hello, world!".to_string()
        );
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(
            transform(
                "Hello, World!",
                Transformation::Uppercase,
                AppType::Unattended
            )
            .unwrap(),
            "HELLO, WORLD!".to_string()
        );
    }

    #[test]
    fn test_no_spaces() {
        assert_eq!(
            transform(
                "Hello, World!",
                Transformation::NoSpaces,
                AppType::Unattended
            )
            .unwrap(),
            "Hello,World!".to_string()
        );
    }

    #[test]
    fn test_slugify() {
        assert_eq!(
            transform(
                "Hello, World!",
                Transformation::Slugify,
                AppType::Unattended
            )
            .unwrap(),
            "hello-world".to_string()
        );
    }

    #[test]
    fn test_double() {
        assert_eq!(
            transform("Hello, World!", Transformation::Double, AppType::Unattended).unwrap(),
            "Hello, World!Hello, World!".to_string()
        );
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            transform(
                "Hello, World!",
                Transformation::Reverse,
                AppType::Unattended
            )
            .unwrap(),
            "!dlroW ,olleH".to_string()
        );
    }

    #[test]
    fn test_empty_string() {
        assert!(transform("", Transformation::Reverse, AppType::Unattended).is_err());
    }

    #[test]
    fn test_newline_string() {
        assert!(transform("\n", Transformation::NoSpaces, AppType::Unattended).is_err());
    }

    #[test]
    fn test_spaces_string() {
        assert!(transform("", Transformation::NoSpaces, AppType::Unattended).is_err());
    }

    #[test]
    fn test_csv_threaded() {
        let expected_output: &str = "\
Country  Population  Capital          
--------------------------------------
USA      331002651   Washington D.C.  
China    1439323776  Beijing          
India    1380004385  New Delhi        
";
        assert_eq!(
            transform("countries.csv", Transformation::Csv, AppType::Threaded).unwrap(),
            expected_output.to_string()
        );
    }

    #[test]
    fn test_csv_unattended() {
        let input_str: &str = "\
Country,Population,Capital
USA,331002651,Washington D.C.
China,1439323776,Beijing
India,1380004385,New Delhi
";

        let expected_output: &str = "\
Country  Population  Capital          
--------------------------------------
USA      331002651   Washington D.C.  
China    1439323776  Beijing          
India    1380004385  New Delhi        
";
        assert_eq!(
            transform(input_str, Transformation::Csv, AppType::Unattended).unwrap(),
            expected_output.to_string()
        );
    }
}
