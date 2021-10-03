use anyhow::{Result};
use chrono::{Local};
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::{io::Write};

mod data;
mod utility;
pub mod controller;

use data::{postgres_handler::PostgresHandler};

use crate::controller::chronicle_controller;

#[rocket::main]
async fn main() -> Result<()>{
    Builder::new()
    .target(Target::Stdout)
    .format(|buf, record| -> Result<(), std::io::Error> {
        writeln!(buf,
            "{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
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

    info!("LOREMASTER: Launching http server...");
    rocket::build()
    .manage(postgres_service)
    // .mount("/", )
    .mount("/chronicle", chronicle_controller::routes())
    .launch()
    .await?;
    
    info!("LOREMASTER: Shutting down...");
    return Ok(());
}
