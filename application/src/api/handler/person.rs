use anyhow::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::{
    entity::{action::Action, person::PersonMeta},
    query::{
        action::{
            create_action::create_action_query, get_action::get_action_query,
            get_all_actions::get_all_actions_query,
        },
        person::{meta_by_id::meta_by_id_query, update_meta_by_id::update_meta_by_id_query},
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
    email_address: &str,
    alias: &str,
) -> Result<PersonMeta> {
    Ok(update_meta_by_id_query(&database_pool, &person_id, &email_address, &alias).await?)
}

pub enum UniqueEntryResult {
    Created,
    Exists,
}

pub async fn create_action(
    database_pool: &Pool<Postgres>,
    action: &String,
) -> Result<UniqueEntryResult> {
    let potential_action: Option<Action> = get_action_query(&database_pool, &action).await?;
    if potential_action.is_some() {
        return Ok(UniqueEntryResult::Exists);
    }
    create_action_query(&database_pool, &action).await?;
    Ok(UniqueEntryResult::Created)
}

pub async fn get_action_list_handler(database_pool: &Pool<Postgres>) -> Result<Vec<Action>> {
    Ok(get_all_actions_query(&database_pool).await?)
}
