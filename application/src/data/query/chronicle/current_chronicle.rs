use anyhow::{Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CURRENT_CHRONICLE_QUERY: &str = "
    SELECT DISTINCT
        chronicle.id
        , chronicle.date_recorded
    FROM
        public.chronicle
    WHERE
        chronicle.date_recorded = CURRENT_DATE
    LIMIT 1
    ;";

pub async fn get_current_chronicle_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
) -> Result<Option<Chronicle>> {
    let query_result = database_connection
        .query_opt(CURRENT_CHRONICLE_QUERY, &[])
        .await
        .context("An error occurred while querying the database.".to_string())?;

    if let Some(chronicle) = query_result {
        return Ok(Some(Chronicle {
            id: chronicle.get::<_, Uuid>("id"),
            date_recorded: Utc.from_utc_datetime(
                &chronicle
                    .get::<_, NaiveDate>("date_recorded")
                    .and_hms(0, 0, 0),
            ),
        }));
    } else {
        return Ok(None);
    }
}
