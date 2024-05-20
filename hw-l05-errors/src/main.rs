use std::env::args;

mod str_utils;
use str_utils::run;

fn main() {
    // Parse cli arguments
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of arguments!");
    } else {
        let result = run(&args[1]);

        match result {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("Error '{e}' occurred!"),
        }
    }
}
