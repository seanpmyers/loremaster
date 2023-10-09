use anyhow::{anyhow, Result};
use log::info;
use sqlx::{query, PgPool};

use crate::data::entity::{person::PersonId, sleep_schedule::SleepScheduleId};

const QUERY: &str = "
INSERT INTO
    public.person_sleep_schedule (
        person_id
        , sleep_schedule_id
    )
VALUES 
    ($1, $2)
;";

pub async fn update_person_sleep_schedule_query(
    database_connection: &PgPool,
    schedule_id: &SleepScheduleId,
    person_id: &PersonId,
) -> Result<()> {
    info!("QUERY CALL: update_person_sleep_schedule_query");

    let updated_row_count: u64 = query(QUERY)
        .bind(person_id)
        .bind(schedule_id)
        .execute(database_connection)
        .await?
        .rows_affected();

    if updated_row_count < 1_u64 {
        return Err(anyhow!(
            "No rows were updated! query: update_person_sleep_schedule_query"
        ));
    }

    Ok(())
}
