use anyhow::Result;
use sqlx::{query, PgPool};
use uuid::Uuid;

const QUERY: &str = "
    DELETE FROM
        public.chronicle
    WHERE
      chronicle.id = $1
    ;";

pub async fn delete_chronicle_query(
    database_connection: &PgPool,
    chronicle_id: &Uuid,
) -> Result<()> {
    query(QUERY)
        .bind(&chronicle_id)
        .execute(database_connection)
        .await?;

    Ok(())
}
