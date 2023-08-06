use std::{thread, time::Duration};

use anyhow::Result;
use configuration::application::LoremasterWebServerConfiguration;
use log::info;

mod api;
mod configuration;
mod data;
mod security;
mod utility;

#[tokio::main]
async fn main() -> Result<()> {
    info!("Starting up!");
    let configuration: LoremasterWebServerConfiguration =
        configuration::application::configure().await?;

    match configuration.scheduler_state {
        configuration::application::SchedulerState::On => {
            let (web_server_result, scheduler_result) = tokio::join!(
                tokio::spawn(async move { api::web_server::start(configuration).await }),
                tokio::spawn(async move { scheduler().await })
            );

            web_server_result.unwrap_err();
            scheduler_result.unwrap_err();
        }
        configuration::application::SchedulerState::Off => {
            api::web_server::start(configuration).await?
        }
    }

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
