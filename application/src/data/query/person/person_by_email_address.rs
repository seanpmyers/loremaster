use anyhow::Result;
use email_address::EmailAddress;
use sqlx::{query_as, PgPool};

use crate::data::entity::person::Person;

const _QUERY: &str = "
SELECT
   email_address
   , creation_date
FROM
   public.person
INNER JOIN person_email_address ON person.id = person_email_address.person_id
INNER JOIN email_address ON person_email_address.email_address_id = email_address.id
WHERE
   email_address.local_part = $1
   AND email_address.domain = $2
LIMIT 
   1
;";

pub async fn person_by_email_address_query(
    database_connection: &PgPool,
    email_address: &EmailAddress,
) -> Result<Option<Person>> {
    let query_result: Option<Person> = query_as::<_, Person>(_QUERY)
        .bind(email_address.local_part())
        .bind(email_address.domain())
        .fetch_optional(database_connection)
        .await?;
    Ok(query_result)
}
