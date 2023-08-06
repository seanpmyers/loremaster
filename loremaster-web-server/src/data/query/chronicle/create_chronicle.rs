use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const CREATE_CHRONICLE_QUERY: &str = "
    INSERT INTO
        public.chronicle (person_id, date_recorded, creation_time)
    VALUES 
        ($1, $2, $3)
    RETURNING
        id
        , person_id
        , date_recorded
        , notes
        , creation_time
    ;";

const CREATE_CHRONICLE_QUERY_WITH_ID: &str = "
    INSERT INTO
        public.chronicle (id, person_id, date_recorded, creation_time)
    VALUES 
    ($1, $2, $3, $4)
    RETURNING
        id
        , person_id
        , date_recorded
        , notes
        , creation_time
    ;";

pub async fn create_chronicle_query(
    database_connection: &PgPool,
    chronicle_date: &Date,
    chronicle_time: &OffsetDateTime,
    person_id: &Uuid,
    chronicle_id: &Option<Uuid>,
) -> Result<Chronicle> {
    info!("QUERY CALL: create_chronicle_query");
    match chronicle_id {
        Some(id) => {
            let query_result: Chronicle = query_as::<_, Chronicle>(CREATE_CHRONICLE_QUERY_WITH_ID)
                .bind(id)
                .bind(person_id)
                .bind(chronicle_date)
                .bind(chronicle_time)
                .fetch_one(database_connection)
                .await?;

            Ok(query_result)
        }
        None => {
            let query_result: Chronicle = query_as::<_, Chronicle>(CREATE_CHRONICLE_QUERY)
                .bind(person_id)
                .bind(chronicle_date)
                .bind(chronicle_time)
                .fetch_one(database_connection)
                .await?;

            Ok(query_result)
        }
    }
}
