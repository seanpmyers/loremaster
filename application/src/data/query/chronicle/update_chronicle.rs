use anyhow::{Context, Result};
use chrono::{NaiveDateTime, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Row, Statement};
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const UPDATE_CHRONICLE_QUERY: &str = "
    UPDATE
        public.chronicle
    SET 
      date_recorded = (TO_DATE($1, 'YYYY-MM-DD'))
    WHERE
      id = $2
   RETURNING
      id
      , date_recorded
    ;";

pub async fn update_chronicle_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    chronicle_to_update: &Chronicle,
) -> Result<Chronicle> {
    let prepared_statement: Statement = database_connection.prepare(UPDATE_CHRONICLE_QUERY).await?;

    let query_result: Row = database_connection
        .query_one(
            &prepared_statement,
            &[
                &chronicle_to_update.date_recorded.to_string(),
                &chronicle_to_update.id,
            ],
        )
        .await
        .context("An error occurred while querying the database.".to_string())?;

    let updated_chronicle: Chronicle = Chronicle {
        id: query_result.get::<_, Uuid>("id"),
        date_recorded: Utc
            .from_local_datetime(&query_result.get::<_, NaiveDateTime>("date_recorded"))
            .unwrap(),
    };

    return Ok(updated_chronicle);
}
