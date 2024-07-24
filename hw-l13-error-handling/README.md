# Homework 13 - Error Handling

## Objective

- [x] Integrate Anyhow and Thiserror
  - Introduce the `anyhow` crate to manage errors in a straightforward, flexible way. This crate is especially useful for handling errors that don't need much context or are unexpected.
  - Utilize the `thiserror` crate to create custom, meaningful error types for your application. This is particularly beneficial for errors where you need more context and structured data.
- [x] Error Handling in the Server
  - Ensure that your server accurately reports errors to the client in a strongly-typed manner. Any operation that can fail should communicate its failure reason clearly and specifically.
- [x] Client-Side Error Management
  - Modify the client to handle and display error messages received from the server appropriately. Ensure that these messages are user-friendly and informative.
- [x] Refactoring for Error Handling
  - Review your existing codebase for both the client and server. Identify areas where error handling can be improved and implement changes using `anyhow` and `thiserror`.
  - Pay special attention to operations that involve network communication, file handling, and data parsing, as these are common sources of errors.
- [x] Documentation and Testing
  - Test various failure scenarios to ensure that errors are handled gracefully and the error messages are clear and helpful.

## Running the example

To run the server and client(s), follow these instructions. Ensure that you start the server binary
before running the client binaries. Each binary should be run in a separate terminal.

- localhost on default port 11111

  ``` bash
  cargo run --bin server
  cargo run --bin client
  ```

- localhost on specified port

  ``` bash
  cargo run --bin server 1234
  cargo run --bin client 1234
  ```

- server on all addresses on specified port

  ``` bash
  cargo run --bin server 1234 0.0.0.0
  cargo run --bin client 1234
  ```

- multiple clients on default `localhost:11111`

  ``` bash
  cargo run --bin server
  cargo run --bin client
  cargo run --bin client
  ```

### Functional requests

- `.file file.txt` -> saves `files/file.txt`
- `.image rust.png` -> saves `images/rust.png`
- `just string` -> returns "just string"
- `.quit` -> terminates connection

### Non-functional requests

- `.file non-existing` -> returns "Error reading file"
- `.image non-existing.png` -> returns "Error reading image"
- `.image wrong-suffix.jpg` -> returns "Wrong image extension. Only PNG files are supported."
