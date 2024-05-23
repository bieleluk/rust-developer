# Homework 7

## Objective

- [x] Spin up two threads: one dedicated to receiving input and another for processing it
- [x] Use channels to transfer data between threads
- [x] Input-Receiving Thread continuously reads from stdin and parse the received input in the format `<command> <input>`
- [x] Processing Thread analyzes received command received from the input thread and executes the appropriate operation
- [x] Adapt application to read CSV from a file using the `read_to_string()` function and handle any potential errors
- [x] Bonus: Oneshot Functionality - the program enters the interactive mode only when there are no CLI arguments

## Running the example

The program can be executed in two modes based on the number of CLI arguments.

### Unattended mode

* The program takes exactly one argument (transformation) and an iput data from the standard input, transforms the data and prints the result to the standard output.

* The program has to be executed with one extra argument:

    ```sh
    cargo run -- <transformation>
    ```
    where the `<transformation>` has to be one of:
    `lowercase` , `uppercase` , `no-spaces` , `slugify` , `double` , `reverse` , `csv`

* In case of a non-existing command or wrong input data, the error message is printed to the standard error stream.

* Use a pipe to run the script and immediately provide multi-line string for the standard input, i.e.:

    ```sh
    echo "Name,Age,City
    Alice,30,New York
    Bob,25,Los Angeles" | cargo run -- csv
    ```

### Interactive mode

* The program indefinitely accepts the commands and data from the standard input in the following format: `<command> <input>`, transforms the data accordingly and prints the result to the standard output.
* In case of `csv` command, the `<input>` should be the path to the existing csv file.
* For other possible transformations, `<input>` is the string to be processed.  
* The program has to be executed with no extra arguments:

    ```sh
    cargo run
    ```

* In case of a non-existing command or wrong input data, the error message is printed to the standard error stream and the cycle repeats.


## Running the tests

* Project contains 15 unit and 2 integration tests to ensure the expected behaviour of each function as well as the entire flow of unattended type.

* They could be executed as follows:

    ```sh
    cargo test
    ```
