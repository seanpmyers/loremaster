use anyhow::Result;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Statement};
use uuid::Uuid;

const QUERY: &str = "
    DELETE FROM
        public.chronicle
    WHERE
      chronicle.id = $1
    ;";

pub async fn delete_chronicle_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    chronicle_id: &Uuid,
) -> Result<()> {
    let prepared_statement: Statement = database_connection.prepare(QUERY).await?;

    database_connection
        .query(&prepared_statement, &[&chronicle_id])
        .await?;

    Ok(())
}
