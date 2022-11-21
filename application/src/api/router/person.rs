use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::Form;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        guards::user::User,
        handler::person::{
            create_action, get_action_list_handler, get_person_meta_data, update_person_meta_data,
            UniqueEntryResult,
        },
        response::ApiError,
    },
    data::{
        entity::{action::Action, person::PersonMeta},
        postgres_handler::PostgresHandler,
    },
};

pub async fn meta(
    postgres_service: Extension<PostgresHandler>,
    user: User,
) -> Result<Response, ApiError> {
    let result: Option<PersonMeta> =
        get_person_meta_data(&postgres_service.database_pool, &user.0).await?;
    match result {
        Some(person) => Ok((StatusCode::OK, Json(person)).into_response()),
        None => Ok((
            StatusCode::NOT_FOUND,
            "Unable to find person with the specified id.",
        )
            .into_response()),
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdatePersonMetaForm {
    email_address: String,
    alias: String,
}

pub async fn update_meta(
    postgres_service: Extension<PostgresHandler>,
    user: User,
    Form(form): Form<UpdatePersonMetaForm>,
) -> Result<Response, ApiError> {
    let sanitized_email_address: &str = form.email_address.trim();
    let sanitized_alias: &str = form.alias.trim();

    let result: PersonMeta = update_person_meta_data(
        &postgres_service.database_pool,
        &user.0,
        sanitized_email_address,
        sanitized_alias,
    )
    .await?;
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(Deserialize, Debug)]
pub struct NewActionForm {
    action: String,
}

pub async fn new_action(
    postgres_service: Extension<PostgresHandler>,
    _user: User,
    Form(form): Form<NewActionForm>,
) -> Result<Response, ApiError> {
    let sanitized_action = form.action.trim().to_ascii_lowercase();
    let result = create_action(&postgres_service.database_pool, &sanitized_action).await?;
    match result {
        UniqueEntryResult::Created => {
            Ok((StatusCode::CREATED, "New action successfully created!").into_response())
        }
        UniqueEntryResult::Exists => {
            Ok((StatusCode::ALREADY_REPORTED, "Action already exists.").into_response())
        }
    }
}

pub async fn get_action_list(
    postgres_service: Extension<PostgresHandler>,
    _user: User,
) -> Result<Response, ApiError> {
    let result: Vec<Action> = get_action_list_handler(&postgres_service.database_pool).await?;
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompoundingInterestInputs {
    pub duration_in_years: u16,
    pub start_age: u8,
    pub initial_amount: f32, // TODO: need 64?
    pub annual_percent_interest: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompoundingInterestResult {
    pub duration_in_years: u16,
}

pub async fn compounding_interest_calculator(
    Form(input_form): Form<CompoundingInterestInputs>,
) -> Result<Response, ApiError> {
    let result: CompoundingInterestResult = CompoundingInterestResult {
        duration_in_years: input_form.duration_in_years,
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub fn router() -> Router {
    Router::new()
        .route("/person/meta", get(meta))
        .route("/person/update/meta", post(update_meta))
        .route("/action/new", post(new_action))
        .route("/action/list", get(get_action_list))
        .route(
            "/person/compounding-interest-calculator",
            get(compounding_interest_calculator),
        )
}
