use anyhow::Result;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use rocket::fs::FileServer;
use sqlx::types::time::OffsetDateTime;
use std::io::Write;

mod api;
mod data;
mod utility;

use data::postgres_handler::PostgresHandler;

use crate::{
    api::controller::{chronicle_controller, session_controller},
    utility::constants::{files::FRONTEND_DIST_PATH, LOCAL_DEBUG, PROFILE},
};

#[rocket::main]
async fn main() -> Result<()> {
    Builder::new()
        .target(Target::Stdout)
        .format(|buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "LOREMASTER_{}: {} [{}] - {}",
                std::env::var(PROFILE).unwrap_or_else(|_| LOCAL_DEBUG.to_string()),
                OffsetDateTime::now_utc().format("%FT%T"),
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
    let _rocket = rocket::build()
        .manage(postgres_service)
        .mount("/", session_controller::routes())
        .mount("/", FileServer::from(FRONTEND_DIST_PATH))
        .mount("/chronicle", chronicle_controller::routes())
        .ignite()
        .await?
        .launch()
        .await?;

    info!("Shutting down.");

    Ok(())
}
