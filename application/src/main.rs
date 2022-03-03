use anyhow::Result;
use chrono::{offset, SecondsFormat};
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::io::Write;

mod api;
pub mod controller;
mod data;
pub mod guards;
mod utility;

use data::postgres_handler::PostgresHandler;

use crate::{
    controller::{chronicle_controller, session_controller},
    utility::constants::{LOCAL_DEBUG, PROFILE},
};

#[rocket::main]
async fn main() -> Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "LOREMASTER_{}: {} [{}] - {}",
                std::env::var(PROFILE).unwrap_or(LOCAL_DEBUG.to_string()),
                offset::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    info!("Starting up.");

    info!("Configuring database connection.");
    let postgres_service: PostgresHandler = PostgresHandler::new().await?;
    info!("Connection configured.");

    info!("Launching rocket HTTP server.");
    rocket::build()
        .manage(postgres_service)
        .mount("/", session_controller::routes())
        .mount("/chronicle", chronicle_controller::routes())
        .launch()
        .await?;

    info!("Shutting down.");
    return Ok(());
}
