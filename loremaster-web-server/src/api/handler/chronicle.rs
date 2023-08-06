use crate::data::{
    entity::{
        chronicle::{current_server_time, get_date_from_timezone, Chronicle},
        transfer::person_chronicle::PersonChronicle,
    },
    query::{
        self,
        chronicle::{
            create_chronicle::create_chronicle_query,
            current_chronicle_by_person::get_current_chronicle_by_person_query,
        },
    },
};
use anyhow::{anyhow, Result};
use log::info;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn handle_get_today(
    database_pool: &Pool<Postgres>,
    person_id: &Uuid,
    requested_timezone_string: &Option<String>,
) -> Result<PersonChronicle> {
    let requested_date: OffsetDateTime = match requested_timezone_string {
        Some(input) => {
            let sanitized_timezone_string: &str = input.trim();
            get_date_from_timezone(current_server_time()?, sanitized_timezone_string)?
        }
        None => current_server_time()?,
    };

    let person_alias: Option<String> =
        query::person::alias_by_id::alias_by_id_query(database_pool, person_id)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    let chronicle_query_result: Option<Chronicle> =
        get_current_chronicle_by_person_query(database_pool, &requested_date, person_id)
            .await
            .map_err(|error| anyhow!("{}", error))?;

    let chronicle: Chronicle = match chronicle_query_result {
        Some(existing_chronicle) => existing_chronicle,
        None => {
            info!("No chronicle exits for the current date. Creating one.");
            let new_chronicle_id: Uuid = Uuid::new_v4();
            create_chronicle_query(
                database_pool,
                &requested_date.date(),
                &requested_date,
                person_id,
                &Some(new_chronicle_id),
            )
            .await
            .map_err(|error| anyhow!("{}", error))?
        }
    };

    Ok(PersonChronicle {
        chronicle_id: chronicle.id,
        chronicle_date: chronicle.date_recorded,
        person_alias,
    })
}
