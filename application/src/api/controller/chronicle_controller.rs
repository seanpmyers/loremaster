use anyhow::anyhow;
use chrono::{offset, DateTime, SecondsFormat, Utc};
use log::info;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use rocket::{delete, get, post, routes, serde::json::Json, State};
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::{
    api::{guards::user::User, response::ApiError},
    data::{
        entity::chronicle::Chronicle,
        postgres_handler::PostgresHandler,
        query::chronicle::{
            chronicle_by_date::chronicle_by_date_query, chronicle_by_id::chronicle_by_id_query,
            create_chronicle::create_chronicle_query,
            current_chronicle::get_current_chronicle_query,
            current_chronicle_by_person::get_current_chronicle_by_person_query,
            delete_chronicle::delete_chronicle_query, update_chronicle::update_chronicle_query,
        },
    },
};

#[get("/today")]
pub async fn today(
    postgres_service: &State<PostgresHandler>,
    user: User,
) -> Result<Json<Chronicle>, ApiError> {
    info!("Connecting to database.");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    info!("Connected to database.");
    info!("Querying for today's chronicle.");
    let today: DateTime<Utc> = offset::Utc::now();

    let current_chronicle_query_result: Option<Chronicle> =
        get_current_chronicle_query(&database_connection)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    if current_chronicle_query_result.is_none() {
        info!("No chronicle exits for the current date. Creating one.");
        let new_chronicle_id: Uuid = Uuid::new_v4();
        create_chronicle_query(
            &database_connection,
            &today,
            &user.0,
            &Some(new_chronicle_id),
        )
        .await
        .map_err(|error| anyhow!("{}", error))?;
    }

    let query_result = get_current_chronicle_by_person_query(&database_connection, &user.0)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    match query_result {
        Some(result) => {
            info!("Existing chronicle found!");
            Ok(Json(result))
        }
        None => {
            info!("User is not associated with today's chronicle. Generating new a relation.");
            let result = create_chronicle_query(&database_connection, &today, &user.0, &None)
                .await
                .map_err(|error| anyhow!("{}", error))?;
            Ok(Json(result))
        }
    }
}

#[get("/by_date")]
pub async fn by_date(
    postgres_service: &State<PostgresHandler>,
    user: User,
) -> Result<Option<Json<Chronicle>>, ApiError> {
    info!("Connecting to database.");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    let chronicle_date: DateTime<Utc> = offset::Utc::now();

    let query_result: Option<Chronicle> =
        chronicle_by_date_query(&database_connection, &chronicle_date, &user.0)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    if let Some(result) = query_result {
        Ok(Some(Json(result)))
    } else {
        Ok(None)
    }
}

#[get("/by_id")]
pub async fn by_id(
    postgres_service: &State<PostgresHandler>,
) -> Result<Option<Json<Chronicle>>, ApiError> {
    info!("Connecting to database.");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    let chronicle_id: Uuid = Uuid::new_v4();

    let query_result: Option<Chronicle> =
        chronicle_by_id_query(&database_connection, &chronicle_id)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    match query_result {
        Some(result) => Ok(Some(Json(result))),
        None => Ok(None),
    }
}

#[post("/update")]
pub async fn update(
    postgres_service: &State<PostgresHandler>,
) -> Result<Json<Chronicle>, ApiError> {
    info!("Connecting to database.");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    let chronicle: Chronicle = Chronicle {
        id: Uuid::new_v4(),
        date_recorded: offset::Utc::now(),
    };

    let query_result: Chronicle = update_chronicle_query(&database_connection, &chronicle)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    Ok(Json(query_result))
}

#[delete("/delete")]
pub async fn delete(postgres_service: &State<PostgresHandler>) -> Result<(), ApiError> {
    info!("Connecting to database.");
    let database_connection: Connection<PgConnectionManager<NoTls>> = postgres_service
        .database_pool
        .get()
        .await
        .map_err(|error| anyhow!("{}", error))?;

    let chronicle_id: Uuid = Uuid::new_v4();

    delete_chronicle_query(&database_connection, &chronicle_id)
        .await
        .map_err(|error| anyhow!("{}", error))?;

    Ok(())
}

#[get("/server_time")]
pub fn server_time() -> Result<String, ApiError> {
    Ok(Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true))
}

#[get("/example")]
pub fn example() -> Result<Json<Chronicle>, ApiError> {
    let result: Chronicle = Chronicle {
        id: Uuid::new_v4(),
        date_recorded: Utc::now(),
    };
    Ok(Json(result))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![by_date, by_id, today, update, delete, server_time, example]
}
