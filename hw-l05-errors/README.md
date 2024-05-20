# Homework 5

## Objective

- [x] Examine the first argument to identify the required operation, and execute its function
- [x] Display the operation's output or an error
- [x] Create a dedicated function for each operation which validates arguments, parses, and returns the output
- [x] Return `Result<String, Box<dyn Error>>`
- [x] Present the selected operation and any errors encountered
- [x] Add additional operation labeled `csv`, that interprets the input string as CSV
- [x] Exhibit the parsed content in an orderly table layout
- [x] Bonus: Hhandle any length of values and headers
- [x] create a Csv struct and implement the Display trait

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

* Project contains 14 unit and 2 integration tests to ensure the expected behaviour of each function as well as the entire flow
* They could be executed as follows:

    ```sh
    cargo test
    ```
