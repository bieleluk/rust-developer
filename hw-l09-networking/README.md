# Homework 9 - Networking

## Objective

- [ ] Design the server to receive messages from multiple clients.
  - Accept port and hostname as parameters. If none are provided, default to `localhost:11111`.
  - Setting the hostname to `0.0.0.0` will allow connections from any IP.
- [ ] Design the client that can connect to the server to send messages.
  - It should accept port and hostname parameters, defaulting to `localhost:11111` if not given.
- [ ] Client should read input from stdin and recognize three distinct message types:
  - `.file <path>`: Sends a file to the server.
  - `.image <path>`: Sends an image (assumed or required to be `.png`).
  - Any other text: Considered a standard text message.
  - The `.quit` command should terminate the client.

- [ ] Server-side File Handling
  - Received images should be stored in the `images/` directory, named by `<timestamp>.png`.
  - Other received files should be stored in the `files/` directory.
  - Incoming text messages should be displayed directly in stdout.
  - Notifications `Receiving image...` and `Receiving <filename>` are displayed for incoming files.

## Running the example

``` bash
cargo run
```
