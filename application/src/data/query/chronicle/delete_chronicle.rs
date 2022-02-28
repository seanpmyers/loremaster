use anyhow::{Context, Result};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

const DELETE_CHRONICLE_QUERY: &str = "
    DELETE FROM
        public.chronicle
    WHERE
      chronicle.id = $1
    ;";

pub async fn delete_chronicle_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    chronicle_id: &Uuid,
) -> Result<()> {
    database_connection
        .query(DELETE_CHRONICLE_QUERY, &[&chronicle_id])
        .await
        .context("An error occurred while querying the database.".to_string())?;
    return Ok(());
}
