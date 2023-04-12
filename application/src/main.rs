use std::{thread, time::Duration};

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

    // tokio::spawn(async move { scheduler().await });

    // let result = tokio::join!(
    //     tokio::spawn(async move { api::web_server::start().await }),
    //     tokio::spawn(async move { scheduler().await })
    // );

    // result.0.unwrap_err();
    // result.1.unwrap_err();

    api::web_server::start().await?;

    info!("Shutting down.");

    Ok(())
}

async fn scheduler() -> Result<()> {
    loop {
        thread::sleep(Duration::from_secs(2));
        info!("Checking schedules...");
    }

    Ok(())
}
