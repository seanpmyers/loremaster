use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CREATE_CHRONICLE_QUERY: &str = "
    INSERT INTO
        public.chronicle (person_id, date_recorded, creation_time)
    VALUES 
        ($1, TO_DATE($2, 'YYYY-MM-DD'), $3)
    RETURNING
        id
    ;";

const CREATE_CHRONICLE_QUERY_WITH_ID: &str = "
    INSERT INTO
        public.chronicle (id, person_id, date_recorded, creation_time)
    VALUES 
    ($1, $2, TO_DATE($3, 'YYYY-MM-DD'), $3)
    RETURNING
        id
    ;";

pub async fn create_chronicle_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    chronicle_date: &DateTime<Utc>,
    person_id: &Uuid,
    chronicle_id: &Option<Uuid>,
) -> Result<Chronicle> {
    match chronicle_id {
        Some(id) => {
            let query_result = database_connection
                .query_one(
                    CREATE_CHRONICLE_QUERY_WITH_ID,
                    &[&id, &person_id, &chronicle_date.to_string()],
                )
                .await
                .context("An error occurred while querying the database.".to_string())?;

            let result_id: Uuid = query_result.get::<_, Uuid>("id");

            let new_chronicle: Chronicle = Chronicle {
                id: result_id,
                date_recorded: chronicle_date.clone(),
            };

            return Ok(new_chronicle);
        }
        None => {
            let query_result = database_connection
                .query_one(
                    CREATE_CHRONICLE_QUERY,
                    &[&person_id, &chronicle_date.to_string()],
                )
                .await
                .context("An error occurred while querying the database.".to_string())?;

            let result_id: Uuid = query_result.get::<_, Uuid>("id");

            let new_chronicle: Chronicle = Chronicle {
                id: result_id,
                date_recorded: chronicle_date.clone(),
            };

            return Ok(new_chronicle);
        }
    }
}
