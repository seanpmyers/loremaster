use anyhow::{
    Context
};
use chrono::{
    offset, 
    Utc, 
    DateTime
};
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{
    State,
    serde::json::Json, 
    get, 
    routes, 
    post, 
    delete, http::Status, 
};
use tokio_postgres::NoTls;
use uuid::Uuid;


use crate::{data::{
   entity::chronicle::Chronicle,
   postgres_handler::PostgresHandler, 
   query::chronicle::{
      create_chronicle::create_chronicle_query, 
      current_chronicle::get_current_chronicle_query, 
      chronicle_by_date::{chronicle_by_date_query}, 
      delete_chronicle::delete_chronicle_query, 
      update_chronicle::update_chronicle_query, chronicle_by_id::chronicle_by_id_query
   }, 
}, guards::user::User};

#[get("/")]
pub async fn today(
    postgres_service: &State<PostgresHandler>, 
    user: User
) -> Json<Chronicle> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();

    info!("LOREMASTER: Connected to database.");
    info!("LOREMASTER: Querying for today's chronicle.");
    let today: DateTime<Utc> = offset::Utc::now();

    let query_result = get_current_chronicle_query(&database_connection, &user.0)
        .await
        .context(format!("Failed to execute query for current chronicle!"))
        .unwrap();
    
    match query_result {
        Some(result) => {
            info!("LOREMASTER: Existing chronicle found!"); 
            return Json(result);
        }
        None => {
            info!("LOREMASTER: No chronicle found for today. Generating new chronicle...");
            let result = create_chronicle_query(&database_connection, &today, &None)
                .await
                .context(format!("Failed to execute create new chronicle query!"))
                .unwrap();
            return Json(result);
        }
    }
}

#[get("/by_date")]
pub async fn by_date(
    postgres_service: &State<PostgresHandler>
) -> Option<Json<Chronicle>> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();

    let chronicle_date = offset::Utc::now();

    let query_result = chronicle_by_date_query(&database_connection, &chronicle_date)
        .await
        .unwrap();

    if let Some(result) = query_result {
        return Some(Json(result))
    }
    else {
        return None;
    }
}

#[get("/by_id")]
pub async fn by_id(
    postgres_service: &State<PostgresHandler>
) -> Option<Json<Chronicle>> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service.database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();
    let chronicle_id = Uuid::new_v4();
    let query_result = chronicle_by_id_query(&database_connection, &chronicle_id)
        .await
        .unwrap();
    match query_result {
        Some(result) => return Some(Json(result)),
        None => return None
    }
}

#[post("/update")]
pub async fn update(
    postgres_service: &State<PostgresHandler>
) -> Json<Chronicle> {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();

    let chronicle = Chronicle{
        id: Uuid::new_v4(),
        date_recorded: offset::Utc::now(),
    };

    let query_result = update_chronicle_query(&database_connection, &chronicle)
        .await
        .unwrap();
    return Json(query_result);
}

#[delete("/delete")]
pub async fn delete(
    postgres_service: &State<PostgresHandler>
) -> Status {
    info!("LOREMASTER: Connecting to database...");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .context(format!("Failed to get database connection!"))
        .unwrap();

    let chronicle_id: Uuid = Uuid::new_v4();

    delete_chronicle_query(&database_connection, &chronicle_id)
        .await
        .unwrap();
    return Status::Ok;
}

#[get("/server_time")]
pub fn server_time() -> Json<DateTime<Utc>> {
    return Json(Utc::now());
}

#[get("/example")]
pub fn example() -> Json<Chronicle> {
    let result: Chronicle = Chronicle {
        id: Uuid::new_v4(),
        date_recorded: Utc::now(),
    };
    return Json(result);
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        by_date,
        by_id,
        today,
        update,
        delete,
        server_time,
        example
    ]
 }