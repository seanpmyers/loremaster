use anyhow::{Context, Result};
use chrono::{Date, NaiveDate, TimeZone, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const UPDATE_CHRONICLE_QUERY : &str = "
    UPDATE
        public.chronicle (date_recorded)
    SET 
      date_recorded = (TO_DATE($1, 'YYYY-MM-DD'))
    WHERE
      id = '$1'
   RETURNING
      id
      , date_recorded
    ;";

pub async fn update_chronicle_query(database_connection: &Connection<PgConnectionManager<NoTls>>, chronicle_to_update: &Chronicle) -> Result<Chronicle> {
    let query_result = database_connection.query_one(UPDATE_CHRONICLE_QUERY, &[&chronicle_to_update.date_recorded.to_string(), &chronicle_to_update.id.to_string()])
    .await.context(format!("An error occurred while querying the database."))?;
    let result_id: Uuid = query_result.get::<_, Uuid>("id");
    let result_date: Date<Utc> = Utc.from_utc_date(&query_result.get::<_, NaiveDate>("date_recorded"));

    let updated_chronicle: Chronicle = Chronicle{
        id: result_id,
        date_recorded: result_date
    };

    return Ok(updated_chronicle);
}