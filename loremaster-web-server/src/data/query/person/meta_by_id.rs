use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::person::{PersonId, PersonMeta};

const QUERY: &str = "
SELECT
   person.id
	 , email_address.display as email_address
	 , registration_date
	 , alias
FROM
   public.person
INNER JOIN
   public.email_address ON email_address.id = person.email_address_id
WHERE
   person.id = $1
LIMIT 
   1
;";

pub async fn meta_by_id_query(
    database_connection: &PgPool,
    person_id: &PersonId,
) -> Result<Option<PersonMeta>> {
    let query_result: Option<PersonMeta> = query_as::<_, PersonMeta>(QUERY)
        .bind(person_id)
        .fetch_optional(database_connection)
        .await?;
    Ok(query_result)
}
