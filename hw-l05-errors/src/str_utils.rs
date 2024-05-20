use csv::ReaderBuilder;
use slug::slugify;
use std::error::Error;
use std::fmt;
use std::io::{stdin, Read};

#[derive(PartialEq, Debug)]
pub struct Csv {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Csv {
    fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Csv { headers, rows }
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Calculate column widths
        let mut column_widths: Vec<usize> =
            self.headers.iter().map(|header| header.len()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if let Some(width) = column_widths.get_mut(i) {
                    if *width < cell.len() {
                        *width = cell.len();
                    }
                }
            }
        }

        // Print headers
        for (i, header) in self.headers.iter().enumerate() {
            if let Some(width) = column_widths.get(i) {
                write!(f, "{:<width$}", header, width = width + 2)?;
            }
        }
        writeln!(f)?;

        // Print separator line
        for width in &column_widths {
            write!(f, "{:-<width$}", "", width = width + 2)?;
        }
        writeln!(f)?;

        // Print rows
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if let Some(width) = column_widths.get(i) {
                    write!(f, "{:<width$}", cell, width = width + 2)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn run(transformation: &str) -> Result<String, Box<dyn Error>> {
    // Read a string from stdin
    let transformation = check_transformation(transformation)?;
    let mut input_str = String::new();
    println!("Insert string and press the Enter");

    // Handle CSV case requiring multi-line input
    match transformation {
        "csv" => stdin().read_to_string(&mut input_str)?,
        _ => stdin().read_line(&mut input_str)?, // valid_transmutation() guarantees no bad inputs
    };

    transform(&input_str, transformation)
}

pub fn check_transformation(transformation: &str) -> Result<&str, Box<dyn Error>> {
    match transformation {
        // Compulsory transformations
        "lowercase" | "uppercase" | "no-spaces" | "slugify" | "double" | "reverse" | "csv" => {
            Ok(transformation)
        }
        _ => Err(From::from("Non-existing transformation")), // Default case for any other value
    }
}

pub fn transform(input_str: &str, transformation: &str) -> Result<String, Box<dyn Error>> {
    match transformation {
        // Compulsory transformations
        "lowercase" => to_lowercase(input_str),
        "uppercase" => to_uppercase(input_str),
        "no-spaces" => remove_spaces(input_str),
        "slugify" => slugify_str(input_str),
        "csv" => csv_formatted_str(csv_parse(input_str)?),
        // Bonus transofrmations
        "double" => double_str(input_str),
        "reverse" => reverse(input_str),
        _ => unreachable!(),
    }
}

fn to_lowercase(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.to_lowercase())
    }
}

fn to_uppercase(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.to_uppercase())
    }
}

fn remove_spaces(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.replace(' ', ""))
    }
}

fn slugify_str(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(slugify(line))
    }
}

fn double_str(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(format!("{}{}", line, line))
    }
}

fn reverse(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.chars().rev().collect())
    }
}

fn csv_parse(input_str: &str) -> Result<Csv, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(input_str.as_bytes());

    // Read headers
    let headers = reader.headers()?.iter().map(|h| h.to_string()).collect();

    // Read rows
    let mut rows = vec![];
    for record in reader.records() {
        let row = record?;
        let row_values = row.iter().map(|v| v.to_string()).collect();
        rows.push(row_values);
    }

    Ok(Csv::new(headers, rows))
}

fn csv_formatted_str(input_csv: Csv) -> Result<String, Box<dyn Error>> {
    Ok(format!("{input_csv}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase() {
        assert_eq!(
            transform("Hello, World!", "lowercase").unwrap(),
            "hello, world!".to_string()
        );
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(
            transform("Hello, World!", "uppercase").unwrap(),
            "HELLO, WORLD!".to_string()
        );
    }

    #[test]
    fn test_no_spaces() {
        assert_eq!(
            transform("Hello, World!", "no-spaces").unwrap(),
            "Hello,World!".to_string()
        );
    }

    #[test]
    fn test_slugify() {
        assert_eq!(
            transform("Hello, World!", "slugify").unwrap(),
            "hello-world".to_string()
        );
    }

    #[test]
    fn test_double() {
        assert_eq!(
            transform("Hello, World!", "double").unwrap(),
            "Hello, World!Hello, World!".to_string()
        );
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            transform("Hello, World!", "reverse").unwrap(),
            "!dlroW ,olleH".to_string()
        );
    }

    #[test]
    fn test_empty_string() {
        assert!(transform("", "reverse").is_err());
    }

    #[test]
    fn test_newline_string() {
        assert!(transform("\n", "no-spaces").is_err());
    }

    #[test]
    fn test_spaces_string() {
        assert!(transform("", "no-spaces").is_err());
    }

    #[test]
    fn test_existing_transformation() {
        assert_eq!(check_transformation("no-spaces").unwrap(), "no-spaces");
    }

    #[test]
    fn test_csv_transformation() {
        assert_eq!(check_transformation("csv").unwrap(), "csv");
    }

    #[test]
    fn test_non_existing_transformation() {
        assert!(check_transformation("adhoc").is_err());
    }

    #[test]
    fn test_csv_display() {
        let headers = vec!["Name", "Age", "City"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let rows = vec![
            vec!["Alice", "308989", "New York"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            vec!["Bob", "25", "Los Angeles"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        ];

        let csv = Csv::new(headers, rows);

        // Generate expected CSV string
        let expected_output: &str = "\
Name   Age     City         
----------------------------
Alice  308989  New York     
Bob    25      Los Angeles  
";

        assert_eq!(csv_formatted_str(csv).unwrap(), expected_output);
    }

    #[test]
    fn test_csv_parse() {
        let headers = vec!["Name", "Age", "City"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let rows = vec![
            vec!["Alice", "30", "New York"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            vec!["Bob", "25", "Los Angeles"]
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        ];

        let csv = Csv::new(headers, rows);

        let input_str = "Name,Age,City\nAlice,30,New York\nBob,25,Los Angeles";

        assert_eq!(csv, csv_parse(input_str).unwrap());
    }
}
