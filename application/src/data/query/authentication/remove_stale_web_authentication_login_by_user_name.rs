use anyhow::Result;
use sqlx::{query, PgPool};

const QUERY: &str = "
	DELETE
	FROM
		public.web_authentication_login
	WHERE
		web_authentication_login.user_name = $1
;";

pub async fn remove_stale_web_authentication_login_by_user_name_query(
    database_connection: &PgPool,
    user_name: &str,
) -> Result<()> {
    query(QUERY)
        .bind(user_name)
        .execute(database_connection)
        .await?;

    Ok(())
}
