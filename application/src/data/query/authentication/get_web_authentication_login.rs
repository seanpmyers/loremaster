use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::web_authentication_challenge::{
    WebAuthenticationChallenge, WebAuthenticationLogin,
};

const QUERY: &str = "
	SELECT
		web_authentication_login.*
	FROM
		public.web_authentication_login
	WHERE
   	web_authentication_login.user_name = $1
;";

pub async fn get_web_authentication_login_query(
    database_connection: &PgPool,
    user_name: &str,
) -> Result<Option<WebAuthenticationLogin>> {
    let query_result: Option<WebAuthenticationChallenge> = query_as(QUERY)
        .bind(user_name)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
