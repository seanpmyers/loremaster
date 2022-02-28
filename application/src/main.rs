use anyhow::Result;
use chrono::offset;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::io::Write;

mod api;
pub mod controller;
mod data;
pub mod guards;
mod utility;

use data::postgres_handler::PostgresHandler;

use crate::controller::{chronicle_controller, session_controller};

#[rocket::main]
async fn main() -> Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "{} [{}] - {}",
                offset::Utc::now().to_rfc3339(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    info!("LOREMASTER: Starting up...");

    info!("LOREMASTER: Configuring database connection...");
    let postgres_service: PostgresHandler = PostgresHandler::new().await?;
    info!("LOREMASTER: Connection configured.");

    info!("LOREMASTER: Launching rocket http server...");
    rocket::build()
        .manage(postgres_service)
        .mount("/", session_controller::routes())
        .mount("/chronicle", chronicle_controller::routes())
        .launch()
        .await?;

    info!("LOREMASTER: Shutting down...");
    return Ok(());
}
