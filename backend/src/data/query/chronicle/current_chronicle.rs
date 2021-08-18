use anyhow::{Context, Result};
use chrono::{TimeZone, Local};
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
    WHERE
        chronicle.date_recorded = CURRENT_DATE
    ;";


pub async fn get_current_chronicle_query(database_connection: &Connection<PgConnectionManager<NoTls>>) -> Result<Option<Chronicle>> {
   let query_result = database_connection.query(CURRENT_CHRONICLE_QUERY, &[]).await.context(format!("An error occurred while querying the database."))?;
   if query_result.len() == 0 { return Ok(None);}
   
   match query_result.get(0) {
       Some(chronicle_result) => {
           let result = Chronicle {
               id: chronicle_result.get::<_, Uuid>("id"),
               date_recorded: Local.from_local_date(&chronicle_result.get::<_, chrono::NaiveDate>("date_recorded")).unwrap()
           };
           return Ok(Some(result));
       }
       None => {
           return Ok(None);
       }
   }
}