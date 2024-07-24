# Homework 9 - Networking

## Objective

- [x] Design the server to receive messages from multiple clients.
  - Accept port and hostname as parameters. If none are provided, default to `localhost:11111`.
  - Setting the hostname to `0.0.0.0` will allow connections from any IP.
- [x] Design the client that can connect to the server to send messages.
  - It should accept port and hostname parameters, defaulting to `localhost:11111` if not given.
- [x] Client should read input from stdin and recognize three distinct message types:
  - `.file <path>`: Sends a file to the server.
  - `.image <path>`: Sends an image (assumed or required to be `.png`).
  - Any other text: Considered a standard text message.
  - The `.quit` command should terminate the client.

- [x] Server-side File Handling
  - Received images should be stored in the `images/` directory, named by `<timestamp>.png`.
  - Other received files should be stored in the `files/` directory.
  - Incoming text messages should be displayed directly in stdout.
  - Notifications `Receiving image...` and `Receiving <filename>` are displayed for incoming files.

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
