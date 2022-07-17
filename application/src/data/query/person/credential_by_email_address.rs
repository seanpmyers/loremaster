use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};

use crate::data::entity::person::Credentials;

const QUERY: &str = "
   SELECT
      id
      , email_address
      , encrypted_password
   FROM
      public.person
   WHERE
      person.email_address = $1
   LIMIT 
      1
;";

pub async fn credential_by_email_address_query(
    database_connection: &PgPool,
    email_address: &String,
) -> Result<Option<Credentials>> {
    info!("QUERY CALL: credential_by_email_address_query");
    let query_result = query_as::<_, Credentials>(QUERY)
        .bind(email_address)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
