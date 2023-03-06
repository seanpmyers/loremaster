use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::Form;
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{
        guards::user::User,
        handler::person::{
            create_action, create_goal, get_action_list_handler, get_frequency_list_handler,
            get_goal_list_handler, get_person_meta_data, get_sleep_schedule_handler,
            remove_one_goal_handler, update_email_handler, update_person_meta_handler,
            update_sleep_schedule_handler, UniqueEntryResult, UserInputValidationOutcome,
        },
        response::ApiError,
    },
    data::{
        entity::{
            action::Action, frequency::Frequency, person::PersonMeta, sleep_schedule::SleepSchedule,
        },
        postgres_handler::PostgresHandler,
    },
    ApplicationState,
};

pub async fn meta(
    State(postgres_service): State<PostgresHandler>,
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
    alias: String,
}

pub async fn update_meta(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Form(form): Form<UpdatePersonMetaForm>,
) -> Result<Response, ApiError> {
    let result: (UserInputValidationOutcome, Option<PersonMeta>) =
        update_person_meta_handler(&postgres_service.database_pool, user.0, form.alias).await?;
    match result {
        (UserInputValidationOutcome::Invalid, _) => Ok((
            StatusCode::BAD_REQUEST,
            Json("Invalid input. Unable to complete your request"),
        )
            .into_response()),
        // TODO: Log malicious input at some point
        (UserInputValidationOutcome::Malicious, _) => Ok((
            StatusCode::BAD_REQUEST,
            Json("Invalid input. Unable to complete your request"),
        )
            .into_response()),
        (UserInputValidationOutcome::Valid, Some(result)) => {
            Ok((StatusCode::OK, Json(result)).into_response())
        }
        (_, _) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Sorry, something went wrong on our side. Please try again."),
        )
            .into_response()),
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdatePersonEmailAddressForm {
    email_address: String,
}

pub async fn update_email_address(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Form(form): Form<UpdatePersonEmailAddressForm>,
) -> Result<Response, ApiError> {
    info!("API Call: update_email_address");
    match update_email_handler(
        &postgres_service.database_pool,
        &user.0,
        &form.email_address,
    )
    .await?
    {
        crate::api::handler::person::EmailAddressUpdateResult::InvalidEmailAddress => {
            Ok((StatusCode::BAD_REQUEST, "Invalid email address").into_response())
        }
        crate::api::handler::person::EmailAddressUpdateResult::EmailInUse => {
            Ok((StatusCode::BAD_REQUEST, "Invalid email address").into_response())
        }
        crate::api::handler::person::EmailAddressUpdateResult::Success => {
            Ok((StatusCode::ACCEPTED, "Email address updated!").into_response())
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct NewActionForm {
    action: String,
}

pub async fn new_action(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Form(form): Form<NewActionForm>,
) -> Result<Response, ApiError> {
    let result: UniqueEntryResult =
        create_action(&postgres_service.database_pool, &user.0, &form.action).await?;
    match result {
        UniqueEntryResult::Created => {
            Ok((StatusCode::CREATED, "New action successfully created!").into_response())
        }
        UniqueEntryResult::Exists => {
            Ok((StatusCode::ALREADY_REPORTED, "Action already exists.").into_response())
        }
        UniqueEntryResult::Added => {
            Ok((StatusCode::OK, "Action successfully added to your list!").into_response())
        }
        UniqueEntryResult::Invalid => {
            Ok((StatusCode::BAD_REQUEST, "Invalid input.").into_response())
        }
    }
}

pub async fn get_action_list(
    State(postgres_service): State<PostgresHandler>,
    _user: User,
) -> Result<Response, ApiError> {
    let result: Vec<Action> = get_action_list_handler(&postgres_service.database_pool).await?;
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(Deserialize, Debug)]
pub struct NewGoalForm {
    goal: String,
}

pub async fn new_goal(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Form(form): Form<NewGoalForm>,
) -> Result<Response, ApiError> {
    let result: UniqueEntryResult =
        create_goal(&postgres_service.database_pool, &user.0, &form.goal).await?;
    match result {
        UniqueEntryResult::Created => {
            Ok((StatusCode::CREATED, "New Goal successfully created!").into_response())
        }
        UniqueEntryResult::Exists => {
            Ok((StatusCode::ALREADY_REPORTED, "Goal already exists.").into_response())
        }
        UniqueEntryResult::Added => {
            Ok((StatusCode::OK, "Goal successfully added to your list!").into_response())
        }
        UniqueEntryResult::Invalid => {
            Ok((StatusCode::BAD_REQUEST, "Invalid input.").into_response())
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GoalQueryParameters {
    goal_id: Uuid,
}

pub async fn remove_goal(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Query(parameters): Query<GoalQueryParameters>,
) -> Result<Response, ApiError> {
    info!("API Call: remove_goal");
    let result: bool = remove_one_goal_handler(
        &postgres_service.database_pool,
        &user.0,
        &parameters.goal_id,
    )
    .await?;
    match result {
        true => Ok((StatusCode::OK, Json("Goal removed!")).into_response()),
        false => Ok((
            StatusCode::BAD_REQUEST,
            Json("Failed to complete the request with the provided input."),
        )
            .into_response()),
    }
}

pub async fn get_goal_list(
    State(postgres_service): State<PostgresHandler>,
    user: User,
) -> Result<Response, ApiError> {
    Ok((
        StatusCode::OK,
        Json(get_goal_list_handler(&postgres_service.database_pool, Some(&user.0)).await?),
    )
        .into_response())
}

pub async fn get_sleep_schedule(
    State(postgres_service): State<PostgresHandler>,
    user: User,
) -> Result<Response, ApiError> {
    let result: Option<SleepSchedule> =
        get_sleep_schedule_handler(&postgres_service.database_pool, &user.0).await?;
    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(Deserialize, Debug)]
pub struct SleepScheduleForm {
    start_time: String,
    end_time: String,
}

pub async fn update_sleep_schedule(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Form(form): Form<SleepScheduleForm>,
) -> Result<Response, ApiError> {
    let result: SleepSchedule = update_sleep_schedule_handler(
        &postgres_service.database_pool,
        &user.0,
        &form.start_time,
        &form.end_time,
    )
    .await?;

    Ok((StatusCode::ACCEPTED, Json(result)).into_response())
}

pub async fn get_frequency_list(
    State(_postgres_service): State<PostgresHandler>,
    _user: User,
) -> Result<Response, ApiError> {
    let frequency_types: Vec<Frequency> = get_frequency_list_handler()?;
    Ok((StatusCode::OK, Json(frequency_types)).into_response())
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

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/person/meta", get(meta))
        .route("/person/sleep-schedule", get(get_sleep_schedule))
        .route("/person/action-list", get(get_action_list))
        .route("/person/frequency-list", get(get_frequency_list))
        .route(
            "/person/compounding-interest-calculator",
            get(compounding_interest_calculator),
        )
        .route("/person/update/meta", post(update_meta))
        .route("/person/update/email_address", post(update_email_address))
        .route("/person/goal-new", post(new_goal))
        .route("/person/goal-remove", post(remove_goal))
        .route("/person/goal-list", get(get_goal_list))
        .route("/person/update/sleep-schedule", post(update_sleep_schedule))
        .route("/person/action-new", post(new_action))
}
