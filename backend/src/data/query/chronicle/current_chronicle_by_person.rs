use anyhow::{Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Row};
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CURRENT_CHRONICLE_QUERY: &str = "
    SELECT DISTINCT
        chronicle.id
        , chronicle.date_recorded
    FROM
        public.chronicle
    INNER JOIN
        public.person_chronicle
        ON chronicle.id = person_chronicle.chronicle_id
    WHERE
        chronicle.date_recorded = CURRENT_DATE
        AND person_chronicle.person_id = $1
    LIMIT 1
    ;";

pub async fn get_current_chronicle_by_person_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    user_id: &Uuid,
) -> Result<Option<Chronicle>> {
    let query_result: Option<Row> = database_connection
        .query_opt(CURRENT_CHRONICLE_QUERY, &[&user_id])
        .await
        .context("An error occurred while querying the database.".to_string())?;

    if let Some(chronicle) = query_result {
        let result = Chronicle {
            id: chronicle.get::<_, Uuid>("id"),
            date_recorded: Utc.from_utc_datetime(
                &chronicle
                    .get::<_, NaiveDate>("date_recorded")
                    .and_hms(0, 0, 0),
            ),
        };
        return Ok(Some(result));
    } else {
        return Ok(None);
    }
}
