use anyhow::{Context, Result};
use chrono::{Date, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const DELETE_CHRONICLE_QUERY : &str = "
    DELETE FROM
        public.chronicle
    WHERE
      chronicle.id = '$1'
    ;";

pub async fn delete_chronicle_query(database_connection: &Connection<PgConnectionManager<NoTls>>, chronicle_id: &Uuid) -> Result<()> {
    database_connection.query(DELETE_CHRONICLE_QUERY, &[&chronicle_id.to_string()]).await.context(format!("An error occurred while deleting chronicle with id: {}.", chronicle_id))?;
    return Ok(());
}

