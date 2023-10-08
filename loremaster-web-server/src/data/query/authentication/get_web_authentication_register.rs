use anyhow::Result;
use log::info;
use sqlx::{query_scalar, PgPool};

use crate::data::entity::web_authentication_challenge::WebAuthenticationChallengeId;

const QUERY: &str = "
    SELECT
        id
    FROM
        public.web_authentication_register
    WHERE
        web_authentication_register.user_name = $1
;";

pub async fn get_optional_web_authentication_id_by_user_name_query(
    database_connection: &PgPool,
    user_name: &str,
) -> Result<Option<WebAuthenticationChallengeId>> {
    info!("QUERY CALL: get_optional_web_authentication_id_by_user_name_query");
    let query_result: Option<WebAuthenticationChallengeId> = query_scalar(QUERY)
        .bind(user_name)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
