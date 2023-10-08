use anyhow::Result;
use log::info;
use sqlx::{query, PgPool};

use crate::data::entity::{person::PersonId, web_authentication_key::WebAuthenticationKeyId};

const QUERY: &str = "
    INSERT INTO
        public.person_web_authentication_key (
            person_id
            , web_authentication_key_id
        )
    VALUES 
        ($1, $2)
    ;";

pub async fn add_web_authentication_key_query(
    database_connection: &PgPool,
    person_id: &PersonId,
    web_authentication_key_id: &WebAuthenticationKeyId,
) -> Result<()> {
    info!("QUERY CALL: add_web_authentication_key_query");

    query(QUERY)
        .bind(person_id)
        .bind(web_authentication_key_id)
        .execute(database_connection)
        .await?;

    Ok(())
}
