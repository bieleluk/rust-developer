use slug::slugify;
use std::error::Error;
use std::io::stdin;

pub fn collect_and_transform(transformation: &str) -> Result<String, Box<dyn Error>> {
    // Read a string from stdin
    let mut line = String::new();
    println!("Insert one-line string and press the Enter");
    stdin().read_line(&mut line)?;
    let line = line.trim();
    transform(line, transformation)
}

pub fn transform(line: &str, transformation: &str) -> Result<String, Box<dyn Error>> {
    match transformation {
        // Compulsory transformations
        "lowercase" => to_lowercase(line),
        "uppercase" => to_uppercase(line),
        "no-spaces" => remove_spaces(line),
        "slugify" => slugify_str(line),
        // Bonus transofrmations
        "double" => double_str(line),
        "reverse" => reverse(line),
        _ => Err(From::from("Non-existing transformation")), // Default case for any other value
    }
}

fn to_lowercase(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input cannot be empty"))
    } else {
        Ok(line.to_lowercase())
    }
}

fn to_uppercase(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input cannot be empty"))
    } else {
        Ok(line.to_uppercase())
    }
}

fn remove_spaces(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input cannot be empty"))
    } else {
        Ok(line.replace(' ', ""))
    }
}

fn slugify_str(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input cannot be empty"))
    } else {
        Ok(slugify(line))
    }
}

fn double_str(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input cannot be empty"))
    } else {
        Ok(format!("{}{}", line, line))
    }
}

fn reverse(line: &str) -> Result<String, Box<dyn Error>> {
    if line.trim().is_empty() {
        Err(From::from("Input cannot be empty"))
    } else {
        Ok(line.chars().rev().collect())
    }
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
    fn test_nonexisting_transformation() {
        assert!(transform("Hello, World!", "unknown").is_err());
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
}
