use anyhow::{Context};
use chrono::Utc;
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{State, get};
use tokio_postgres::NoTls;

use crate::data::{
   postgres_handler::PostgresHandler, 
   query::chronicle::{
      create_chronicle::create_chronicle_query
      , current_chronicle::get_current_chronicle_query
   }
};

#[get("/")]
pub async fn current(postgres_service: &State<PostgresHandler>) -> String {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool.get().await.context(format!("Failed to get database connection!")).unwrap();
    info!("LOREMASTER: Connected to database.");
    info!("LOREMASTER: Querying for today's chronicle.");
    let today = Utc::today();
    let query_result = get_current_chronicle_query(&database_connection).await.context(format!("Failed to execute query for current chronicle!")).unwrap();
    
    match query_result {
        Some(chronicle_result) => {
            info!("LOREMASTER: Existing chronicle found!"); 
            return format!("{}, {}", chronicle_result.id, chronicle_result.date_recorded);
        }
        None => {
            info!("LOREMASTER: No chronicle found for today. Generating new chronicle...");
            let chronicle_result = create_chronicle_query(&database_connection, &today).await.context(format!("Failed to execute create new chronicle query!")).unwrap();
            return format!("{}, {}", chronicle_result.id, chronicle_result.date_recorded);
        }
    }
}