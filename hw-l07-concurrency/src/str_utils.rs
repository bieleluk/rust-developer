use csv::ReaderBuilder;
use slug::slugify;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
            self.headers.iter().map(String::len).collect();
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

pub fn to_lowercase(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.to_lowercase())
    }
}

pub fn to_uppercase(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.to_uppercase())
    }
}

pub fn remove_spaces(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.replace(' ', ""))
    }
}

pub fn slugify_str(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(slugify(line))
    }
}

pub fn double_str(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(format!("{}{}", line, line))
    }
}

pub fn reverse(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input is empty"))
    } else {
        Ok(line.chars().rev().collect())
    }
}

pub fn csv_open(path_str: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(path_str);
    let mut fd = File::open(path)?;
    // Grab the contents and store them as a String to be processed
    let mut csv_str = String::new();
    fd.read_to_string(&mut csv_str)?;

    Ok(csv_str)
}

pub fn csv_parse(input_str: &str) -> Result<Csv, Box<dyn Error>> {
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

pub fn csv_formatted_str(input_csv: Csv) -> Result<String, Box<dyn Error>> {
    Ok(format!("{input_csv}"))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_csv_open() {
        let expected_str = "Name,Age,City\nAlice,30,New York\nBob,25,Los Angeles";

        assert_eq!(csv_open("people.csv").unwrap(), expected_str);
    }

    #[test]
    fn test_csv_nonexisting_file_open() {

        assert!(csv_open("loremipsum.csv").is_err());
    }
}
