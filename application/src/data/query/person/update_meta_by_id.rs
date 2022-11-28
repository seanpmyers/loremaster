use anyhow::{anyhow, Result};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::data::entity::person::PersonMeta;

const QUERY: &str = "
UPDATE
	person
SET
	alias = $2
WHERE
	id = $1
;";

const RESULT_QUERY: &str = "
    SELECT
        person.id
        , email_address.display as email_address
        , person.registration_date
        , person.alias
    FROM
        public.person
    INNER JOIN
        public.email_address ON person.email_address_id = email_address.id
    WHERE
        person.id = $1
    LIMIT
        1
;";

pub async fn update_meta_by_id_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    alias: &str,
) -> Result<PersonMeta> {
    let updated_row_count: u64 = query(QUERY)
        .bind(&person_id)
        .bind(&alias)
        .execute(database_connection)
        .await?
        .rows_affected();

    if updated_row_count < 1_u64 {
        return Err(anyhow!(
            "No rows were updated! query: update_meta_by_id_query"
        ));
    }

    let query_result: PersonMeta = query_as::<_, PersonMeta>(RESULT_QUERY)
        .bind(&person_id)
        .fetch_one(database_connection)
        .await?;
    Ok(query_result)
}
