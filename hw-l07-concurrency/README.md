# Homework 7

## Objective

- [x] Spin up two threads: one dedicated to receiving input and another for processing it
- [ ] Use channels to transfer data between threads
- [ ] Input-Receiving Thread continuously reads from stdin and parse the received input in the format `<command> <input>`
- [ ] Processing Thread analyzes received command received from the input thread and executes the appropriate operation
- [ ] Adapt application to read CSV from a file using the `read_to_string()` function and handle any potential errors
- [x] Bonus: Oneshot Functionality - the program enters the interactive mode only when there are no CLI arguments

## Running the example

* The program should be executed with one extra argument:

    ```sh
    cargo run -- <transformation>
    ```
    where the `<transformation>` has to be one of:
    `lowercase` , `uppercase` , `no-spaces` , `slugify` , `double` , `reverse` , `csv`

* Use a pipe to run the script and immediately provide string for the standard input, i.e.:

    ```sh
    echo "Name,Age,City
    Alice,30,New York
    Bob,25,Los Angeles" | cargo run -- csv
    ```

## Running the tests

* Project contains 11 unit and 2 integration tests to ensure the expected behaviour of each function as well as the entire flow
* They could be executed as follows:

    ```sh
    cargo test
    ```
