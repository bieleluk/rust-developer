use std::env::args;
use std::process::exit;
mod str_utils;
use str_utils::run;

fn main() {
    // Parse cli arguments
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments!");
        exit(1);
    } else {
        let result = run(&args[1]);

        match result {
            Ok(result) => println!("{result}"),
            Err(e) => {
                eprintln!("Error '{e}' occurred!");
                exit(1);
            }
        }
    }
}
