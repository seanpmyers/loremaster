use anyhow::Result;
use log::info;
use sqlx::{query, PgPool};
use uuid::Uuid;

const QUERY: &str = "
    INSERT INTO
			public.password (
					id
					, encrypted_password
			)
    VALUES 
    ($1, $2)
    ;";

pub async fn add_password_query(
    database_connection: &PgPool,
    encrypted_password: &String,
) -> Result<Uuid> {
    info!("QUERY CALL: add_password_query");
    let new_password_id: Uuid = Uuid::new_v4();

    query(QUERY)
        .bind(new_password_id)
        .bind(encrypted_password)
        .execute(database_connection)
        .await?;

    Ok(new_password_id)
}
