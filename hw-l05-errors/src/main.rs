use std::env::args;
use std::io;

mod str_utils;
use str_utils::transform;

fn main() {
    let mut line = String::new();

    // Parse cli arguments
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments!");
    } else {
        println!("Enter a string:");
        // Read a string from stdin
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();
        println!("Original: {line}");

        let transformed = transform(&line, &args[1]);
        match transformed {
            Some(result) => println!("Transformed result: {}", result),
            None => eprintln!("Unknown argument!"),
        }
    }
}
