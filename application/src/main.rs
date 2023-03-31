use anyhow::Result;
use log::info;

mod api;
mod configuration;
mod data;
mod security;
mod utility;

#[tokio::main]
async fn main() -> Result<()> {
    configuration::application::configure().await?;
    info!("Starting up!");

    api::web_server::start().await?;

    info!("Shutting down.");

    Ok(())
}
