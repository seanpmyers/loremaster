use std::str::FromStr;

use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};
use time::{format_description::FormatItem, macros::format_description, Time};
use uuid::Uuid;

use crate::{
    data::{
        entity::{
            self, action::Action, frequency::Frequency, goal::Goal, person::PersonMeta,
            sleep_schedule::SleepSchedule,
        },
        query::{
            action::{
                create_action::create_action_query, get_action_by_name::get_action_by_name_query,
                get_all_actions::get_all_actions_query,
            },
            email_address::create_email_address::create_email_address_query,
            frequency::get_frequency_list::get_frequency_list_query,
            goal::{
                create_goal::create_goal_query, get_goal_by_name::get_goal_by_name_query,
                get_goal_list::get_goal_list_query,
            },
            person::{
                action_is_related::action_is_related_query, add_action::add_action_query,
                add_goal::add_goal_query,
                credential_by_email_address::credential_by_email_address_query,
                get_person_sleep_schedule::get_person_sleep_schedule_query,
                goal_is_related::goal_is_related_query, meta_by_id::meta_by_id_query,
                remove_one_goal::remove_one_goal_query,
                update_email_address::update_email_address_query,
                update_meta_by_id::update_meta_by_id_query,
                update_person_sleep_schedule::update_person_sleep_schedule_query,
            },
            sleep_schedule::{
                create_sleep_schedule::create_sleep_schedule_query,
                get_sleep_schedule_by_time::get_sleep_schedule_by_time_query,
            },
        },
    },
    security::sanitization::sanitize_user_input_string,
};

pub enum UserInputValidationOutcome {
    Invalid,
    Valid,
}

pub enum UniqueEntryResult {
    Created,
    Exists,
    Added,
    Invalid,
}

pub enum EmailAddressUpdateResult {
    InvalidEmailAddress,
    EmailInUse,
    Success,
}

pub async fn get_person_meta_data(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
) -> Result<Option<PersonMeta>> {
    Ok(meta_by_id_query(&database_pool, &person_id).await?)
}

pub async fn get_sleep_schedule_handler(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
) -> Result<Option<SleepSchedule>> {
    let potential_sleep_schedule: Option<SleepSchedule> =
        get_person_sleep_schedule_query(&database_pool, &person_id).await?;

    Ok(potential_sleep_schedule)
}

pub fn get_frequency_list_handler() -> Result<Vec<Frequency>> {
    Ok(get_frequency_list_query()?)
}

pub async fn get_action_list_handler(database_pool: &Pool<Postgres>) -> Result<Vec<Action>> {
    Ok(get_all_actions_query(&database_pool).await?)
}

pub async fn get_goal_list_handler(
    database_pool: &Pool<Postgres>,
    person_id: Option<&Uuid>,
) -> Result<Vec<Goal>> {
    Ok(get_goal_list_query(&database_pool, person_id).await?)
}

pub async fn create_action(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    action: &String,
) -> Result<UniqueEntryResult> {
    let sanitized_action: String = action.trim().to_ascii_lowercase();

    if action.len() == 0 {
        return Ok(UniqueEntryResult::Invalid);
    }

    let potential_action: Option<Action> =
        get_action_by_name_query(&database_pool, &sanitized_action).await?;

    match potential_action {
        Some(action) => {
            let relation_exists: bool =
                action_is_related_query(&database_pool, &person_id, &action.id).await?;
            match relation_exists {
                true => Ok(UniqueEntryResult::Exists),
                false => {
                    add_action_query(&database_pool, &person_id, &action.id).await?;
                    Ok(UniqueEntryResult::Added)
                }
            }
        }
        None => {
            let action: Action = create_action_query(&database_pool, &sanitized_action).await?;
            add_action_query(&database_pool, &person_id, &action.id).await?;
            Ok(UniqueEntryResult::Created)
        }
    }
}

pub async fn create_goal(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    goal: &String,
) -> Result<UniqueEntryResult> {
    let sanitized_goal: String = goal.trim().to_ascii_lowercase();

    if goal.len() == 0 {
        return Ok(UniqueEntryResult::Invalid);
    }

    let potential_goal: Option<Goal> =
        get_goal_by_name_query(&database_pool, &sanitized_goal).await?;

    match potential_goal {
        Some(goal) => {
            let relation_exists: bool =
                goal_is_related_query(&database_pool, &person_id, &goal.id).await?;
            match relation_exists {
                true => Ok(UniqueEntryResult::Exists),
                false => {
                    add_goal_query(&database_pool, &person_id, &goal.id).await?;
                    Ok(UniqueEntryResult::Added)
                }
            }
        }
        None => {
            let goal: Goal = create_goal_query(&database_pool, &goal).await?;
            add_goal_query(&database_pool, &person_id, &goal.id).await?;
            Ok(UniqueEntryResult::Created)
        }
    }
}

pub async fn update_person_meta_handler(
    database_pool: &Pool<Postgres>,
    person_id: Uuid,
    alias: String,
) -> Result<(UserInputValidationOutcome, Option<PersonMeta>)> {
    let sanitized_alias: String = sanitize_user_input_string(alias.clone())?;

    // Alias cannot be an empty string
    if sanitized_alias.is_empty() {
        return Ok((UserInputValidationOutcome::Invalid, None));
    }

    Ok((
        UserInputValidationOutcome::Valid,
        Some(update_meta_by_id_query(&database_pool, &person_id, &sanitized_alias).await?),
    ))
}

pub async fn update_email_handler(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    email_address: &String,
) -> Result<EmailAddressUpdateResult> {
    let sanitized_email_address: String = email_address.trim().to_ascii_lowercase();

    if !email_address::EmailAddress::is_valid(&sanitized_email_address) {
        return Ok(EmailAddressUpdateResult::InvalidEmailAddress);
    }

    let valid_email_address: email_address::EmailAddress =
        email_address::EmailAddress::from_str(&sanitized_email_address)
            .map_err(|error| anyhow!("{}", error))?;

    if credential_by_email_address_query(&database_pool, &valid_email_address)
        .await?
        .is_some()
    {
        return Ok(EmailAddressUpdateResult::EmailInUse);
    }

    let new_email_address: entity::email_address::EmailAddress =
        create_email_address_query(&database_pool, &valid_email_address).await?;

    update_email_address_query(&database_pool, &person_id, &new_email_address.id).await?;

    Ok(EmailAddressUpdateResult::Success)
}

pub async fn update_sleep_schedule_handler(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    start_time: &String,
    end_time: &String,
) -> Result<SleepSchedule> {
    let format: &[FormatItem] = format_description!("[hour]:[minute]");
    let start_time = Time::parse(&start_time.trim().to_ascii_lowercase(), format)?;
    let end_time = Time::parse(&end_time.trim().to_ascii_lowercase(), format)?;

    let potential_existing_schedule: Option<SleepSchedule> =
        get_sleep_schedule_by_time_query(&database_pool, &start_time, &end_time).await?;

    match potential_existing_schedule {
        Some(schedule) => {
            update_person_sleep_schedule_query(&database_pool, &schedule.id, &person_id).await?;
            Ok(schedule)
        }
        None => {
            let new_schedule: SleepSchedule =
                create_sleep_schedule_query(&database_pool, &start_time, &end_time).await?;
            update_person_sleep_schedule_query(&database_pool, &new_schedule.id, &person_id)
                .await?;
            Ok(new_schedule)
        }
    }
}

pub async fn remove_one_goal_handler(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    goal_id: &Uuid,
) -> Result<bool> {
    let relation_exists: bool = goal_is_related_query(&database_pool, &person_id, &goal_id).await?;
    match relation_exists {
        true => {
            remove_one_goal_query(&database_pool, &person_id, &goal_id).await?;
            Ok(true)
        }
        false => Ok(false),
    }
}
