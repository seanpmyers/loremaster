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

use crate::{
    api::{
        guards::user::User,
        handler::person::{
            create_action, create_goal, get_action_list_handler, get_goal_list_handler,
            get_person_meta_data, get_sleep_schedule_handler, remove_one_goal_handler,
            update_email_handler, update_person_meta_handler, update_sleep_schedule_handler,
            UniqueEntryResult, UserInputValidationOutcome,
        },
        response::ApiError,
        web_server::ApplicationState,
    },
    data::{
        entity::{
            action::Action,
            goal::GoalId,
            person::{PersonId, PersonMeta},
            sleep_schedule::SleepSchedule,
        },
        postgres_handler::PostgresHandler,
    },
};

pub async fn meta(
    State(postgres_service): State<PostgresHandler>,
    user: User,
) -> Result<Response, ApiError> {
    let result: Option<PersonMeta> =
        get_person_meta_data(&postgres_service.database_pool, &PersonId(user.0)).await?;
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
        &PersonId(user.0),
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
    let result: UniqueEntryResult = create_action(
        &postgres_service.database_pool,
        &PersonId(user.0),
        &form.action,
    )
    .await?;
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
    let result: UniqueEntryResult = create_goal(
        &postgres_service.database_pool,
        &PersonId(user.0),
        &form.goal,
    )
    .await?;
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
    goal_id: GoalId,
}

pub async fn remove_goal(
    State(postgres_service): State<PostgresHandler>,
    user: User,
    Query(parameters): Query<GoalQueryParameters>,
) -> Result<Response, ApiError> {
    info!("API Call: remove_goal");
    let result: bool = remove_one_goal_handler(
        &postgres_service.database_pool,
        &PersonId(user.0),
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
        Json(
            get_goal_list_handler(&postgres_service.database_pool, Some(&PersonId(user.0))).await?,
        ),
    )
        .into_response())
}

pub async fn get_sleep_schedule(
    State(postgres_service): State<PostgresHandler>,
    user: User,
) -> Result<Response, ApiError> {
    let result: Option<SleepSchedule> =
        get_sleep_schedule_handler(&postgres_service.database_pool, &PersonId(user.0)).await?;
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
        &PersonId(user.0),
        &form.start_time,
        &form.end_time,
    )
    .await?;

    Ok((StatusCode::ACCEPTED, Json(result)).into_response())
}

#[derive(Deserialize, Serialize, Debug)]
pub enum InvestmentFrequency {
    Monthly,
    Annually,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompoundingInterestInputs {
    pub duration_in_years: u16,
    pub initial_amount: f32,
    pub percent_interest: f32,
    pub interest_frequency: InvestmentFrequency,
    pub contribution_amount: f32,
    pub contribution_frequency: InvestmentFrequency,
    pub percent_inflation: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InvestmentStatus {
    pub total_value: f32,
    pub total_principal: f32,
    pub interest_value: f32,
    pub contribution_value: f32,
    pub deflated_value: f32,
    pub deflation_difference: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompoundingInterestResult {
    pub inputs: CompoundingInterestInputs,
    pub results: Vec<InvestmentStatus>,
}

pub async fn compounding_interest_calculator(
    Json(input_form): Json<CompoundingInterestInputs>,
) -> Result<Response, ApiError> {
    if input_form.duration_in_years == 0 {
        return Ok((StatusCode::BAD_REQUEST, "Years cannot be zero.").into_response());
    }
    let years = (input_form.duration_in_years + 1) as u32;

    const MONTHS_IN_A_YEAR: u32 = 12;
    const ITERATION_START: u32 = 1;
    const PERCENTAGE_MULTIPLIER: f32 = 0.01_f32;

    let mut results: Vec<InvestmentStatus> = Vec::with_capacity(years as usize);

    let mut current_value: f32 = input_form.initial_amount;
    let mut current_interest_value: f32 = 0_f32;
    let mut current_contribution_value: f32 = 0_f32;

    let iterations: u32 = match (
        &input_form.contribution_frequency,
        &input_form.interest_frequency,
    ) {
        (_, InvestmentFrequency::Monthly) => years * MONTHS_IN_A_YEAR,
        (InvestmentFrequency::Monthly, _) => years * MONTHS_IN_A_YEAR,
        (_, _) => years,
    };

    for index in ITERATION_START..iterations {
        let is_year = match (
            &input_form.contribution_frequency,
            &input_form.interest_frequency,
        ) {
            (InvestmentFrequency::Annually, InvestmentFrequency::Annually) => true,
            (_, _) => index % MONTHS_IN_A_YEAR == 0 && index != 0,
        };

        match (&input_form.interest_frequency, is_year) {
            (InvestmentFrequency::Annually, true) => {
                current_interest_value +=
                    current_value * input_form.percent_interest * PERCENTAGE_MULTIPLIER;
                current_value +=
                    current_value * input_form.percent_interest * PERCENTAGE_MULTIPLIER;
            }
            (InvestmentFrequency::Annually, false) => (),
            (_, _) => {
                current_interest_value +=
                    current_value * input_form.percent_interest * PERCENTAGE_MULTIPLIER;
                current_value +=
                    current_value * input_form.percent_interest * PERCENTAGE_MULTIPLIER;
            }
        }

        match (&input_form.contribution_frequency, is_year) {
            (InvestmentFrequency::Annually, true) => {
                current_contribution_value += &input_form.contribution_amount;
                current_value += &input_form.contribution_amount;
            }
            (InvestmentFrequency::Annually, false) => (),
            (_, _) => {
                current_contribution_value += &input_form.contribution_amount;
                current_value += &input_form.contribution_amount;
            }
        }

        results.push(InvestmentStatus {
            total_value: current_value,
            total_principal: current_value - current_interest_value,
            interest_value: current_interest_value,
            contribution_value: current_contribution_value,
            deflated_value: current_value
                - current_value * input_form.percent_inflation * PERCENTAGE_MULTIPLIER,
            deflation_difference: current_value
                - (current_value
                    - current_value * input_form.percent_inflation * PERCENTAGE_MULTIPLIER),
        });
    }

    let result: CompoundingInterestResult = CompoundingInterestResult {
        inputs: input_form,
        results,
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub fn router() -> Router<ApplicationState> {
    Router::new()
        .route("/person/meta", get(meta))
        .route("/person/sleep-schedule", get(get_sleep_schedule))
        .route("/person/action-list", get(get_action_list))
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
