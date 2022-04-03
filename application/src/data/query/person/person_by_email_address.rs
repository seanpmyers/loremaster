use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::person::Person;

const _QUERY: &str = "
SELECT
   email_address
   , creation_date
FROM
   public.person
WHERE
   person.email_address = $1
LIMIT 
   1
;";

pub async fn _person_by_email_address_query(
    database_connection: &PgPool,
    email_address: &str,
) -> Result<Option<Person>> {
    let query_result: Option<Person> = query_as::<_, Person>(_QUERY)
        .bind(&email_address)
        .fetch_optional(database_connection)
        .await?;
    Ok(query_result)
}
