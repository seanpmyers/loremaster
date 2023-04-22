use anyhow::Result;
use email_address::EmailAddress;
use log::info;
use sqlx::{query_scalar, PgPool};
use uuid::Uuid;

const QUERY: &str = "
   SELECT
      person.id
      , email_address.display as email_address
      , person.encrypted_password
   FROM
      public.email_address
   INNER JOIN
      public.person ON public.email_address.id = person.email_address_id
   WHERE
      email_address.local_part = $1
      AND email_address.domain = $2
   LIMIT 
      1
;";

pub async fn email_address_in_use_query(
    database_connection: &PgPool,
    email_address: &EmailAddress,
) -> Result<bool> {
    info!("QUERY CALL: email_address_in_use_query");
    let query_result: Option<Uuid> = query_scalar(QUERY)
        .bind(email_address.local_part())
        .bind(email_address.domain())
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result.is_some())
}
