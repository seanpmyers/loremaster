use anyhow::anyhow;
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use log::info;
use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    api::{guards::user::User, handler, response::ApiError, web_server::ApplicationState},
    data::{
        entity::{
            chronicle::{current_sever_time_string, Chronicle, ChronicleId},
            person::PersonId,
            transfer::person_chronicle::PersonChronicle,
        },
        postgres_handler::PostgresHandler,
        query::chronicle::{
            chronicle_by_date::chronicle_by_date_query, chronicle_by_id::chronicle_by_id_query,
        },
    },
};

#[derive(Deserialize, Debug)]
pub struct TodayParameters {
    timezone: Option<String>,
}

pub async fn today(
    State(postgres_service): State<PostgresHandler>,
    Query(parameters): Query<TodayParameters>,
    user: User,
) -> Result<Json<PersonChronicle>, ApiError> {
    info!("Querying for today's chronicle.");

    let result: PersonChronicle = handler::chronicle::handle_get_today(
        &postgres_service.database_pool,
        &user.0,
        &parameters.timezone,
    )
    .await?;

    Ok(Json(result))
}

pub async fn by_date(
    State(postgres_service): State<PostgresHandler>,
    user: User,
) -> Result<Json<Option<Chronicle>>, ApiError> {
    let chronicle_date: OffsetDateTime = OffsetDateTime::now_utc();

    let query_result: Option<Chronicle> =
        chronicle_by_date_query(&postgres_service.database_pool, &chronicle_date, &user.0)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    let Some(result) = query_result else {
        return Ok(Json(None));
    };

    Ok(Json(Some(result)))
}

pub async fn by_id(
    State(postgres_service): State<PostgresHandler>,
) -> Result<Json<Option<Chronicle>>, ApiError> {
    let chronicle_id: Uuid = Uuid::new_v4();

    let query_result: Option<Chronicle> =
        chronicle_by_id_query(&postgres_service.database_pool, &chronicle_id)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    match query_result {
        Some(result) => Ok(Json(Some(result))),
        None => Ok(Json(None)),
    }
}

pub async fn server_time() -> Result<String, ApiError> {
    Ok(current_sever_time_string()?)
}

pub async fn example() -> Result<Json<Chronicle>, ApiError> {
    let result: Chronicle = Chronicle {
        id: ChronicleId(Uuid::nil()),
        date_recorded: OffsetDateTime::now_utc().date(),
        person_id: PersonId(Uuid::nil()),
        notes: Some("Here are some notes".to_string()),
        creation_time: Some(OffsetDateTime::now_utc()),
    };
    Ok(Json(result))
}

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/chronicle/server_time", get(server_time))
        .route("/chronicle/example", get(example))
        .route("/chronicle/today", get(today))
        .route("/chronicle/by_date", get(by_date))
        .route("/chronicle/by_id", get(by_id))
}
