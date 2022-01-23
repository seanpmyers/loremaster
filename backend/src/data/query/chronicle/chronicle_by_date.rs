use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CHRONICLE_BY_DATE_QUERY : &str = "
   SELECT
      chronicle.id
      , chronicle.date_recorded
   FROM
      public.chronicle
   WHERE
      chronicle.date_recorded = (TO_DATE($1, 'YYYY-MM-DD'))
   LIMIT 1
    ;";


pub async fn chronicle_by_date_query(
   database_connection: &Connection<PgConnectionManager<NoTls>>, 
   chronicle_date: &DateTime<Utc>
) -> Result<Option<Chronicle>> {
   let query_result = database_connection
      .query_opt(
         CHRONICLE_BY_DATE_QUERY, 
         &[&chronicle_date.to_string()]
      )
      .await
      .context(format!("An error occurred while querying the database."))?;
   match query_result {
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