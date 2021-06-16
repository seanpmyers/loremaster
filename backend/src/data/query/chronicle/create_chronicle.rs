use anyhow::{Context, Result};
use chrono::{Date, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CREATE_CHRONICLE_QUERY : &str = "
    INSERT INTO
        public.chronicle (date_recorded)
    VALUES 
    (TO_DATE($1, 'YYYY-MM-DD'))
    RETURNING
        id
    ;";

pub async fn create_chronicle_query(database_connection: &Connection<PgConnectionManager<NoTls>>, chronicle_date: &Date<Utc>) -> Result<Chronicle> {
    let query_result = database_connection.query_one(CREATE_CHRONICLE_QUERY, &[&chronicle_date.to_string()]).await.context(format!("An error occurred while querying the database."))?;
    let result_id: Uuid = query_result.get::<_, Uuid>("id");

    let new_chronicle: Chronicle = Chronicle{
        id: result_id,
        date_recorded: chronicle_date.clone()
    };

    return Ok(new_chronicle);
}