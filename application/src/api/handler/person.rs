use std::str::FromStr;

use anyhow::{anyhow, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::{
    entity::{self, action::Action, goal::Goal, person::PersonMeta},
    query::{
        action::{
            create_action::create_action_query, get_action_by_name::get_action_by_name_query,
            get_all_actions::get_all_actions_query,
        },
        email_address::create_email_address::create_email_address_query,
        goal::{create_goal::create_goal_query, get_goal_by_name::get_goal_by_name_query},
        person::{
            credential_by_email_address::credential_by_email_address_query,
            meta_by_id::meta_by_id_query, update_email_address::update_email_address_query,
            update_meta_by_id::update_meta_by_id_query,
        },
    },
};

pub async fn get_person_meta_data(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
) -> Result<Option<PersonMeta>> {
    Ok(meta_by_id_query(&database_pool, &person_id).await?)
}

pub async fn update_person_meta_data(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    alias: &str,
) -> Result<PersonMeta> {
    Ok(update_meta_by_id_query(&database_pool, &person_id, &alias).await?)
}

pub enum UniqueEntryResult {
    Created,
    Exists,
    Invalid,
}

pub async fn create_action(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    action: &String,
) -> Result<UniqueEntryResult> {
    if action.len() == 0 {
        return Ok(UniqueEntryResult::Invalid);
    }
    let potential_action: Option<Action> =
        get_action_by_name_query(&database_pool, &action).await?;
    if potential_action.is_some() {
        return Ok(UniqueEntryResult::Exists);
    }
    create_action_query(&database_pool, &action).await?;
    Ok(UniqueEntryResult::Created)
}

pub async fn get_action_list_handler(database_pool: &Pool<Postgres>) -> Result<Vec<Action>> {
    Ok(get_all_actions_query(&database_pool).await?)
}

pub async fn create_goal(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    goal: &String,
) -> Result<UniqueEntryResult> {
    if goal.len() == 0 {
        return Ok(UniqueEntryResult::Invalid);
    }
    let potential_goal: Option<Goal> = get_goal_by_name_query(&database_pool, &goal).await?;
    if potential_goal.is_some() {
        return Ok(UniqueEntryResult::Exists);
    }
    create_goal_query(&database_pool, &goal).await?;
    Ok(UniqueEntryResult::Created)
}

pub async fn update_meta_handler(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    alias: &String,
) -> Result<PersonMeta> {
    let sanitized_alias: &str = alias.trim();

    let result: PersonMeta =
        update_person_meta_data(&database_pool, &person_id, sanitized_alias).await?;
    Ok(result)
}

pub enum EmailAddressUpdateResult {
    InvalidEmailAddress,
    EmailInUse,
    Success,
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
