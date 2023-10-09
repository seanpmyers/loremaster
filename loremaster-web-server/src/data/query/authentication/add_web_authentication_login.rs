use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::web_authentication_challenge::WebAuthenticationLogin;

const QUERY: &str = "
    INSERT INTO 
        public.web_authentication_login (
            id
            , user_name
            , passkey
        )
    VALUES 
        ($1, $2, $3)
    RETURNING
        id
        , user_name
        , passkey
;";

pub async fn add_web_authentication_login_query(
    database_connection: &PgPool,
    web_authentication_challenge: &WebAuthenticationLogin,
) -> Result<WebAuthenticationLogin> {
    let query_result: WebAuthenticationLogin = query_as::<_, WebAuthenticationLogin>(QUERY)
        .bind(&web_authentication_challenge.id)
        .bind(&web_authentication_challenge.user_name)
        .bind(&web_authentication_challenge.passkey)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
