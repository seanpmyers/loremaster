use anyhow::{Context, Result};
use chrono::{TimeZone, NaiveDateTime, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CURRENT_CHRONICLE_QUERY : &str = "
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


pub async fn get_current_chronicle_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    user_id: &Uuid
) -> Result<Option<Chronicle>> {

    let query_result = database_connection
        .query(
            CURRENT_CHRONICLE_QUERY, 
            &[&user_id])
        .await
        .context("An error occurred while querying the database.".to_string())?;

    if query_result.len() == 0 { return Ok(None);}
   
    match query_result.get(0) {
        Some(row) => {
           let result = Chronicle {
               id: row.get::<_, Uuid>("id"),
               date_recorded: Utc
               .from_local_datetime(&row.get::<_, NaiveDateTime>("date_recorded"))
               .unwrap()
           };
            return Ok(Some(result));
        }
        None => {
            return Ok(None);
        }
   }
}