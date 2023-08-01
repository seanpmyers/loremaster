use anyhow::Result;
use log::info;
use sqlx::{query, PgPool};

const QUERY: &str = "
	DELETE
	FROM
		public.web_authentication_register
	WHERE
        web_authentication_register.user_name = $1
;";

pub async fn remove_stale_web_authentication_registers_by_user_name_query(
    database_connection: &PgPool,
    user_name: &str,
) -> Result<()> {
    info!("QUERY CALL: remove_stale_web_authentication_registers_by_user_name_query");
    query(QUERY)
        .bind(user_name)
        .execute(database_connection)
        .await?;

    Ok(())
}
