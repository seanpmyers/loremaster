use anyhow::{anyhow, Result};
use chrono::{NaiveDateTime, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Row, Statement};
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const QUERY: &str = "
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
    let prepared_statement: Statement = database_connection.prepare(QUERY).await?;

    let query_result: Row = database_connection
        .query_one(
            &prepared_statement,
            &[
                &chronicle_to_update.date_recorded.to_string(),
                &chronicle_to_update.id,
            ],
        )
        .await
        .map_err(|error| anyhow!("{}", error))?;

    let updated_chronicle: Chronicle = Chronicle {
        id: query_result.get::<_, Uuid>("id"),
        date_recorded: Utc
            .from_local_datetime(&query_result.get::<_, NaiveDateTime>("date_recorded"))
            .unwrap(),
    };

    Ok(updated_chronicle)
}
