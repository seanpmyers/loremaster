use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use time::Time;

use crate::data::entity::sleep_schedule::SleepSchedule;

const QUERY: &str = "
SELECT
    sleep_schedule.id
    ,sleep_schedule.start_time
    ,sleep_schedule.end_time
FROM
   public.sleep_schedule
WHERE
	sleep_schedule.start_time = $1
	AND sleep_schedule.end_time = $2
LIMIT 
   1
;";

pub async fn get_sleep_schedule_by_time_query(
    database_connection: &PgPool,
    start_time: &Time,
    end_time: &Time,
) -> Result<Option<SleepSchedule>> {
    info!("QUERY CALL: get_sleep_schedule_by_time_query");
    let query_result: Option<SleepSchedule> = query_as::<_, SleepSchedule>(QUERY)
        .bind(start_time)
        .bind(end_time)
        .fetch_optional(database_connection)
        .await?;
    Ok(query_result)
}
