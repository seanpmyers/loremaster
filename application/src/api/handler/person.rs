use anyhow::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::{
    entity::person::PersonMeta,
    query::person::{meta_by_id::meta_by_id_query, update_meta_by_id::update_meta_by_id_query},
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
