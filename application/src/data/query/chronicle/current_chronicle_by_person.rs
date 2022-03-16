use anyhow::{anyhow, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Row, Statement};
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const QUERY: &str = "
    SELECT DISTINCT
        chronicle.id
        , chronicle.date_recorded
    FROM
        public.chronicle
    WHERE
        chronicle.date_recorded = CURRENT_DATE
        AND chronicle.person_id = $1
    LIMIT 1
    ;";

pub async fn get_current_chronicle_by_person_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    person_id: &Uuid,
) -> Result<Option<Chronicle>> {
    let prepared_statement: Statement = database_connection.prepare(QUERY).await?;

    let query_result: Option<Row> = database_connection
        .query_opt(&prepared_statement, &[&person_id])
        .await
        .map_err(|error| anyhow!("{}", error))?;

    if let Some(chronicle) = query_result {
        let result = Chronicle {
            id: chronicle.get::<_, Uuid>("id"),
            date_recorded: Utc.from_utc_datetime(
                &chronicle
                    .get::<_, NaiveDate>("date_recorded")
                    .and_hms(0, 0, 0),
            ),
        };
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
