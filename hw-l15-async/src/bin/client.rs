use anyhow::{Context, Result};
use env_logger;
use log::info;
use networking::client::start_client;
use networking::common::parse_addr;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger
    env_logger::init();

    // Parse port and ipv4 addr from the arguments
    let args: Vec<String> = env::args().collect();
    let (ip, port) = parse_addr(&args[1..]).context("Failed to parse address")?;
    info!("Parsed address is: {}:{}", ip, port);

    // Start the client
    start_client(ip, port)
        .await
        .context("Client execution finished error")?;
    info!("Client execution finished without error");
    Ok(())
}
