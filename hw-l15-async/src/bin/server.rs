use anyhow::{Context, Ok, Result};
use env_logger;
use log::info;
use networking::common::parse_addr;
use networking::server::start_server;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Parse port and ipv4 addr from the arguments
    let args: Vec<String> = env::args().collect();
    let (ip, port) = parse_addr(&args[1..]).context("Failed to parse address")?;
    info!("Parsed address is: {}:{}", ip, port);

    // Start the server
    start_server(ip, port)
        .await
        .context("Server execution finished error")?;
    info!("Server execution finished without error");
    Ok(())
}
