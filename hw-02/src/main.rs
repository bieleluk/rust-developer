use slug::slugify;
use std::env;
use std::io;

fn main() {
    let mut line = String::new();
    println!("Enter a string:");
    // Read a string from stdin
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let line = line.trim();
    println!("Original: {line}");

    // Parse cli arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // Convert arg[1] to &str
        let str_ref: &str = &args[1];
        let transformed = match str_ref {
            "lowercase" => Some(line.to_lowercase()),
            "uppercase" => Some(line.to_uppercase()),
            "no-spaces" => Some(line.replace(" ", "")),
            "slugify" => Some(slugify(&line)),
            _ => None, // Default case for any other value
        };
        match transformed {
            Some(result) => println!("Transformed result: {}", result),
            None => println!("Unknown argument!"),
        }
    } else {
        println!("Wrong number of arguments!")
    }
}
