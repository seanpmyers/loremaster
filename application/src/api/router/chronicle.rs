use anyhow::anyhow;
use axum::{routing::get, Extension, Json, Router};
use log::info;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use uuid::Uuid;

use crate::{
    api::{guards::user::User, response::ApiError},
    data::{
        entity::chronicle::Chronicle,
        postgres_handler::PostgresHandler,
        query::chronicle::{
            chronicle_by_date::chronicle_by_date_query, chronicle_by_id::chronicle_by_id_query,
            create_chronicle::create_chronicle_query,
            current_chronicle_by_person::get_current_chronicle_by_person_query,
        },
    },
};

pub async fn today(
    postgres_service: Extension<PostgresHandler>,
    user: User,
) -> Result<Json<Chronicle>, ApiError> {
    info!("Querying for today's chronicle.");
    let today: OffsetDateTime = OffsetDateTime::now_utc();

    let query_result: Option<Chronicle> =
        get_current_chronicle_by_person_query(&postgres_service.database_pool, &today, &user.0)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    match query_result {
        Some(result) => Ok(Json(result)),
        None => {
            info!("No chronicle exits for the current date. Creating one.");
            let new_chronicle_id: Uuid = Uuid::new_v4();
            let result = create_chronicle_query(
                &postgres_service.database_pool,
                &today.date(),
                &today,
                &user.0,
                &Some(new_chronicle_id),
            )
            .await
            .map_err(|error| anyhow!("{}", error))?;
            Ok(Json(result))
        }
    }
}

pub async fn by_date(
    postgres_service: Extension<PostgresHandler>,
    user: User,
) -> Result<Json<Option<Chronicle>>, ApiError> {
    let chronicle_date: OffsetDateTime = OffsetDateTime::now_utc();

    let query_result: Option<Chronicle> =
        chronicle_by_date_query(&postgres_service.database_pool, &chronicle_date, &user.0)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    if let Some(result) = query_result {
        Ok(Json(Some(result)))
    } else {
        Ok(Json(None))
    }
}

pub async fn by_id(
    postgres_service: Extension<PostgresHandler>,
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
    Ok(OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| anyhow!("{}", error))?)
}

pub async fn example() -> Result<Json<Chronicle>, ApiError> {
    let result: Chronicle = Chronicle {
        id: Uuid::nil(),
        date_recorded: OffsetDateTime::now_utc().date(),
        person_id: Uuid::nil(),
        notes: Some("Here are some notes".to_string()),
        creation_time: Some(OffsetDateTime::now_utc()),
    };
    Ok(Json(result))
}

pub fn router() -> Router {
    Router::new()
        .route("/chronicle/server_time", get(server_time))
        .route("/chronicle/example", get(example))
        .route("/chronicle/today", get(today))
        .route("/chronicle/by_date", get(by_date))
        .route("/chronicle/by_id", get(by_id))
}
