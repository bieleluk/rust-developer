use std::env::args;
use std::process::exit;
mod str_utils;
use std::error::Error;
use str_utils::run;

use std::thread;

fn input_parser() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

fn data_processor() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}

fn main() {
    // Parse cli arguments
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let result = run(&args[1]);

        match result {
            Ok(result) => println!("{result}"),
            Err(e) => {
                eprintln!("Error '{e}' occurred!");
                exit(1);
            }
        }
    } else if args.len() == 1 {
        let handle_input = thread::spawn(input_parser);
        let handle_data = thread::spawn(data_processor);

        let result_input = handle_input.join();
        let result_data = handle_data.join();

        match result_input {
            Ok(Ok(_)) => println!("Input parsing thread completed successfully."),
            Ok(Err(e)) => {
                eprintln!("Error occurred in Input parsing thread: {}", e);
                exit(1);
            }
            Err(e) => {
                eprintln!("Input parsing thread panicked: {:?}", e);
                exit(2);
            }
        }

        match result_data {
            Ok(Ok(_)) => println!("Data processing thread completed successfully."),
            Ok(Err(e)) => {
                eprintln!("Error occurred in data processing thread: {}", e);
                exit(1);
            }
            Err(e) => {
                eprintln!("Input parser thread panicked: {:?}", e);
                exit(2);
            }
        }
    } else {
        eprintln!("Wrong number of arguments!");
        exit(1);
    }
}
