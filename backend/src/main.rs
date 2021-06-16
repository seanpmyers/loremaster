use anyhow::{Context, Result};
use chrono::{Local, Utc};
use env_logger::{Builder, Target};
use log::{LevelFilter, info};
use std::{io::Write};

mod data;
mod utility;

use data::{
    postgres_handler::PostgresHandler, 
    query::chronicle::{
        create_chronicle::create_chronicle_query, 
        current_chronicle::get_current_chronicle_query
    }
};


#[tokio::main]
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

    info!("LOREMASTER: Getting database connection...");
    info!("LOREMASTER: Connecting to database...");
    info!("LOREMASTER: Connected to database.");

    let postgres_context: PostgresHandler = PostgresHandler::new().await.context("Failed to create postgres handler!")?;
    let database_connection = postgres_context.database_pool.get().await.context(format!("Failed to get database connection!"))?;
    
    info!("LOREMASTER: Querying for today's chronicle.");

    let today = Utc::today();
    let query_result = get_current_chronicle_query(&database_connection).await.context(format!("Failed to execute query for current chronicle!"))?;
    
    match query_result {
        Some(chronicle_result) => {
            info!("LOREMASTER: Existing chronicle found!");
            println!("{}, {}", chronicle_result.id, chronicle_result.date_recorded);
        }
        None => {
            info!("LOREMASTER: No chronicle found for today. Generating new chronicle...");
            let chronicle_result = create_chronicle_query(&database_connection, &today).await.context(format!("Failed to execute create new chronicle query!"))?;
            println!("{}, {}", chronicle_result.id, chronicle_result.date_recorded);
        }
    }
       

    info!("LOREMASTER: Shutting down...");
    return Ok(());
}
