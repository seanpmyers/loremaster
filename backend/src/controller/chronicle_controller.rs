use anyhow::{Context};
use chrono::{offset, Utc, DateTime};
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{State, get, routes, post, delete};
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::{
   entity::chronicle::Chronicle,
   postgres_handler::PostgresHandler, 
   query::chronicle::{
      create_chronicle::create_chronicle_query, 
      current_chronicle::get_current_chronicle_query, 
      chronicle_by_date::{chronicle_by_date_query}, 
      delete_chronicle::delete_chronicle_query, 
      update_chronicle::update_chronicle_query, chronicle_by_id::chronicle_by_id_query
   }, 
};

#[get("/")]
pub async fn current(postgres_service: &State<PostgresHandler>) -> Option<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool.get().await.context(format!("Failed to get database connection!")).unwrap();
    info!("LOREMASTER: Connected to database.");
    info!("LOREMASTER: Querying for today's chronicle.");
    let today: DateTime<Utc> = offset::Utc::now();
    let query_result = get_current_chronicle_query(&database_connection).await.context(format!("Failed to execute query for current chronicle!")).unwrap();
    
    match query_result {
        Some(chronicle_result) => {
            info!("LOREMASTER: Existing chronicle found!"); 
            return Some(format!("{}, {}", chronicle_result.id, chronicle_result.date_recorded));
        }
        None => {
            info!("LOREMASTER: No chronicle found for today. Generating new chronicle...");
            let chronicle_result = create_chronicle_query(&database_connection, &today, &None)
                .await.context(format!("Failed to execute create new chronicle query!"))
                .unwrap();
            return Some(format!("{}, {}", chronicle_result.id, chronicle_result.date_recorded));
        }
    }
}

#[get("/by_date")]
pub async fn by_date(postgres_service: &State<PostgresHandler>) -> Option<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool.get().await.context(format!("Failed to get database connection!")).unwrap();
    let chronicle_date = offset::Utc::now();
    let _query_result = chronicle_by_date_query(&database_connection, &chronicle_date).await;
    unimplemented!();
    // return None;
}

#[get("/by_id")]
pub async fn by_id(postgres_service: &State<PostgresHandler>) -> Option<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool.get().await.context(format!("Failed to get database connection!")).unwrap();
    let chronicle_id = Uuid::new_v4();
    let _query_result = chronicle_by_id_query(&database_connection, &chronicle_id).await;
    unimplemented!();
    // return None;
}

#[post("/update")]
pub async fn update_chronicle(postgres_service: &State<PostgresHandler>) -> Option<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool.get().await.context(format!("Failed to get database connection!")).unwrap();

    let chronicle = Chronicle{
        id: Uuid::new_v4(),
        date_recorded: offset::Utc::now(),
    };

    let _query_result = update_chronicle_query(&database_connection, &chronicle).await;
    unimplemented!();
    // return None;
}

#[delete("/delete")]
pub async fn delete_chronicle(postgres_service: &State<PostgresHandler>) -> Option<String> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool.get().await.context(format!("Failed to get database connection!")).unwrap();

    let chronicle_id = Uuid::new_v4();

    let _query_result = delete_chronicle_query(&database_connection, &chronicle_id);
    unimplemented!();
    // return None;
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        by_date,
        by_id,
        current,
        update_chronicle,
        delete_chronicle
    ]
 }