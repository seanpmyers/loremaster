use anyhow::Result;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::data::entity::person::PersonMeta;

const QUERY: &str = "
UPDATE
	person
SET
	email_address = $2,
	alias = $3
WHERE
	id = $1
RETURNING
	id,
	email_address,
    registration_date,
	alias
;";

pub async fn update_meta_by_id_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    email_address: &str,
    alias: &str,
) -> Result<PersonMeta> {
    let query_result: PersonMeta = query_as::<_, PersonMeta>(QUERY)
        .bind(&person_id)
        .bind(&email_address)
        .bind(&alias)
        .fetch_one(database_connection)
        .await?;
    Ok(query_result)
}
