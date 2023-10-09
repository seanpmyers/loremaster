use anyhow::Result;
use log::info;
use sqlx::{query, PgPool};

use crate::data::entity::web_authentication_key::WebAuthenticationKey;

const QUERY: &str = "
    INSERT INTO
			public.web_authentication_key (
					id
					, credential_id
					, cose_algorithm
					, passkey
			)
		VALUES 
    	($1, $2, $3, $4)
    ;";

pub async fn add_web_authentication_key_query(
    database_connection: &PgPool,
    web_authentication_key: &WebAuthenticationKey,
) -> Result<()> {
    info!("QUERY CALL: add_web_authentication_key_query");

    query(QUERY)
        .bind(&web_authentication_key.id)
        .bind(&web_authentication_key.credential_id)
        .bind(web_authentication_key.cose_algorithm)
        .bind(&web_authentication_key.passkey)
        .execute(database_connection)
        .await?;

    Ok(())
}
