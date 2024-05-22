use concurrency::data_processor;
use concurrency::input_parser;
use std::env::args;
use std::process::exit;
use concurrency::run_complete;
use std::thread;

fn main() {
    // Parse cli arguments
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let result = run_complete(&args[1]);

        match result {
            Ok(result) => println!("{result}"),
            Err(e) => {
                eprintln!("Error '{e}' occurred!");
                exit(1);
            }
        }
    } else if args.len() == 1 {
        // Spawn threads for input processing and for transofming the data
        let handle_input = thread::spawn(input_parser);
        let handle_data = thread::spawn(data_processor);
        // Wait until thethreads finish execution
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
