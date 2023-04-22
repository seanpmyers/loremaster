use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::web_authentication_challenge::WebAuthenticationChallenge;

const QUERY: &str = "
    SELECT
        id
        , user_name
        , passkey_registration
    FROM
        public.web_authentication_challenge
    WHERE
        web_authentication_challenge.user_name = $1
    LIMIT 
        1
;";

pub async fn get_web_authentication_by_user_name_query(
    database_connection: &PgPool,
    user_name: &str,
) -> Result<Option<WebAuthenticationChallenge>> {
    let query_result: Option<WebAuthenticationChallenge> = query_as(QUERY)
        .bind(user_name)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
