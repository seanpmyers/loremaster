use anyhow::{Context, Result};
use chrono::{NaiveDate, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CHRONICLE_BY_ID_QUERY : &str = "
   SELECT
      chronicle.id
      , chronicle.date_recorded
   FROM
      public.chronicle
   WHERE
      chronicle.id = $1
   LIMIT 1
    ;";


pub async fn chronicle_by_id_query(database_connection: &Connection<PgConnectionManager<NoTls>>, chronicle_id: &Uuid) -> Result<Option<Chronicle>> {
   let query_result = database_connection.query(CHRONICLE_BY_ID_QUERY, &[&chronicle_id])
   .await.context(format!("An error occurred while querying the database."))?;
   if query_result.len() == 0 { return Ok(None);}
   
   match query_result.get(0) {
       Some(chronicle_result) => {
           let result = Chronicle {
               id: chronicle_result.get::<_, Uuid>("id"),
               date_recorded: Utc.from_utc_date(&chronicle_result.get::<_, NaiveDate>("date_recorded")) 
           };
           return Ok(Some(result));
       }
       None => {
           return Ok(None);
       }
   }
}