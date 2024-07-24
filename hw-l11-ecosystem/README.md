# Homework 11 - Rust Ecosystem

## Objective

- [x] Cargo Crates Conversion
  - If you have not already, transform both the client and server parts of your chat application into separate Cargo crates.
  - Structure your project directory to clearly separate the two parts of the application.
- [x] Shared Functionality
  - Identify any shared functionality between the client and server.
  - Consider abstracting this shared code into a third "library" crate that both the client and server can utilize.
- [x] Production-Ready Libraries
  - Introduce production-ready libraries for key functionalities, such as `log` (with some backend).
- [x] Documentation and Comments
  - Update your `README.md` to document how to use the new crates and any significant changes you've made to the application structure.
  - Add comments throughout your code to explain your reasoning and provide guidance on how the code works.
- [x] Refactoring
  - Refactor your existing codebase to make use of the new crates and shared library, ensuring that everything is cleanly integrated and operates smoothly.

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
