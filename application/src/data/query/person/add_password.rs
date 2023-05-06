use anyhow::Result;
use log::info;
use sqlx::{query, PgPool};
use uuid::Uuid;

const QUERY: &str = "
    INSERT INTO
			public.person_password (
					person_id
					, password_id
			)
    VALUES 
    ($1, $2)
    ;";

pub async fn add_password_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    password_id: &Uuid,
) -> Result<()> {
    info!("QUERY CALL: add_password_query");

    query(QUERY)
        .bind(person_id)
        .bind(password_id)
        .execute(database_connection)
        .await?;

    Ok(())
}
