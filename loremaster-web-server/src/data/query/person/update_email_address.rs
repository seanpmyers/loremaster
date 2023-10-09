use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::{
    email_address::EmailAddressId,
    person::{Person, PersonId},
};

const QUERY: &str = "
UPDATE
	person
SET
	email_address_id = $2
WHERE
	id = $1
RETURNING
	id,
	email_address_id,
    encrypted_password,
	registration_date,
	alias,
    chronicle_id
;";

pub async fn update_email_address_query(
    database_connection: &PgPool,
    person_id: &PersonId,
    email_address_id: &EmailAddressId,
) -> Result<Person> {
    let query_result: Person = query_as::<_, Person>(QUERY)
        .bind(person_id)
        .bind(email_address_id)
        .fetch_one(database_connection)
        .await?;
    Ok(query_result)
}
