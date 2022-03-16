use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Statement};
use uuid::Uuid;

use crate::{
    data::entity::chronicle::Chronicle,
    utility::constants::database::{DATE_RECORDED, ID},
};

const QUERY: &str = "
   SELECT
      chronicle.id
      , chronicle.date_recorded
   FROM
      public.chronicle
   WHERE
      chronicle.date_recorded = $1
      AND chronicle.person_id = $2
   LIMIT 1
    ;";

pub async fn chronicle_by_date_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    chronicle_date: &DateTime<Utc>,
    person_id: &Uuid,
) -> Result<Option<Chronicle>> {
    let prepared_statement: Statement = database_connection.prepare(QUERY).await?;

    let query_result = database_connection
        .query_opt(
            &prepared_statement,
            &[&chronicle_date.to_string(), &person_id],
        )
        .await
        .map_err(|error| anyhow!("{}", error))?;

    match query_result {
        Some(row) => {
            let result = Chronicle {
                id: row.get::<_, Uuid>(ID),
                date_recorded: Utc
                    .from_utc_datetime(&row.get::<_, NaiveDate>(DATE_RECORDED).and_hms(0, 0, 0)),
            };
            Ok(Some(result))
        }
        None => Ok(None),
    }
}
