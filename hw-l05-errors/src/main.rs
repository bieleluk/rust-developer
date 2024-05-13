use std::env::args;

mod str_utils;
use str_utils::collect_and_transform;

fn main() {
    // Parse cli arguments
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments!");
    } else {
        let result = collect_and_transform(&args[1]);

        match result {
            Ok(result) => println!("Transformed result: {}", result),
            Err(e) => eprintln!("Error {e} occurred!"),
        }
    }
}
