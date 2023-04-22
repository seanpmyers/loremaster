use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::web_authentication_challenge::WebAuthenticationChallenge;

const QUERY: &str = "
   INSERT INTO 
        public.web_authentication_challenge (

        )
    VALUES 
        ($1, $2, $3)
   RETURNING
    
;";

pub async fn add_web_authentication_challenge_query(
    database_connection: &PgPool,
    web_authentication_challenge: &WebAuthenticationChallenge,
) -> Result<Option<WebAuthenticationChallenge>> {
    let query_result: Option<WebAuthenticationChallenge> =
        query_as::<_, WebAuthenticationChallenge>(QUERY)
            .bind(&web_authentication_challenge.id)
            .bind(&web_authentication_challenge.user_name)
            .bind(&web_authentication_challenge.passkey_registration)
            .fetch_optional(database_connection)
            .await?;

    Ok(query_result)
}
