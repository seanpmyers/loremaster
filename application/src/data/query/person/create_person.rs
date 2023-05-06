use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::data::entity::person::Person;

const QUERY: &str = "
    INSERT INTO
        public.person (
            id
            , email_address_id
            , registration_date 
            , alias
            , chronicle_id
        )
    VALUES 
    ($1, $2, $3, $4, $5, $6)
    RETURNING
        id
        , email_address_id
        , registration_date 
        , alias
        , chronicle_id
    ;";

pub async fn create_person_query(
    database_connection: &PgPool,
    email_address_id: &Uuid,
    alias: Option<&str>,
    chronicle_id: Option<Uuid>,
) -> Result<Person> {
    info!("QUERY CALL: create_person_query");
    let new_person_id: Uuid = Uuid::new_v4();
    let creation_date: OffsetDateTime = OffsetDateTime::now_utc();

    let query_result: Person = query_as::<_, Person>(QUERY)
        .bind(new_person_id)
        .bind(email_address_id)
        .bind(creation_date)
        .bind(alias)
        .bind(chronicle_id)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
