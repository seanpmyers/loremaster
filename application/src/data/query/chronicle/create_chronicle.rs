use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use log::error;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Statement};
use uuid::Uuid;

use crate::{data::entity::chronicle::Chronicle, utility::constants::database::ID};

const CREATE_CHRONICLE_QUERY: &str = "
    INSERT INTO
        public.chronicle (person_id, date_recorded, creation_time)
    VALUES 
        ($1, TO_DATE($2, 'YYYY-MM-DD'), $3::TIME)
    RETURNING
        id
    ;";

const CREATE_CHRONICLE_QUERY_WITH_ID: &str = "
    INSERT INTO
        public.chronicle (id, person_id, date_recorded, creation_time)
    VALUES 
    ($1, $2, TO_DATE($3, 'YYYY-MM-DD'), $3::TIME )
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
            let prepared_statement: Statement = database_connection
                .prepare(CREATE_CHRONICLE_QUERY_WITH_ID)
                .await?;

            let query_result = database_connection
                .query_one(
                    &prepared_statement,
                    &[&id, &person_id, &chronicle_date.to_string()],
                )
                .await;

            match query_result {
                Ok(row) => {
                    let result_id: Uuid = row.get::<_, Uuid>(ID);

                    let new_chronicle: Chronicle = Chronicle {
                        id: result_id,
                        date_recorded: chronicle_date.clone(),
                    };

                    return Ok(new_chronicle);
                }
                Err(error) => {
                    error!("{}", error);
                    return Err(anyhow!(
                        "Something went wrong while creating a new chronicle."
                    ));
                }
            }
        }
        None => {
            let prepared_statement: Statement =
                database_connection.prepare(CREATE_CHRONICLE_QUERY).await?;

            let query_result = database_connection
                .query_one(
                    &prepared_statement,
                    &[&person_id, &chronicle_date.to_string()],
                )
                .await;

            match query_result {
                Ok(row) => {
                    let result_id: Uuid = row.get::<_, Uuid>(ID);

                    let new_chronicle: Chronicle = Chronicle {
                        id: result_id,
                        date_recorded: chronicle_date.clone(),
                    };

                    return Ok(new_chronicle);
                }
                Err(error) => {
                    error!("{}", error);
                    return Err(anyhow!(
                        "Something went wrong while creating a new chronicle."
                    ));
                }
            }
        }
    }
}
