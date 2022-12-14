use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use time::Time;
use uuid::Uuid;

use crate::data::entity::sleep_schedule::SleepSchedule;

const QUERY: &str = "
    INSERT INTO
        public.sleep_schedule (
            id
            , start_time
            , end_time
        )
    VALUES 
    ($1, $2, $3)
    RETURNING
        id
        , start_time
        , end_time
    ;";

pub async fn create_sleep_schedule_query(
    database_connection: &PgPool,
    start_time: &Time,
    end_time: &Time,
) -> Result<SleepSchedule> {
    info!("QUERY CALL: create_sleep_schedule_query");
    let new_id: Uuid = Uuid::new_v4();

    let query_result: SleepSchedule = query_as::<_, SleepSchedule>(QUERY)
        .bind(&new_id)
        .bind(&start_time)
        .bind(&end_time)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
