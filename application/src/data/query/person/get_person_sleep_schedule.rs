use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::data::entity::sleep_schedule::SleepSchedule;

const QUERY: &str = "
SELECT
	sleep_schedule.id
	, sleep_schedule.start_time
	, sleep_schedule.end_time
FROM
	public.sleep_schedule
INNER JOIN
	public.person_sleep_schedule ON sleep_schedule.id = person_sleep_schedule.sleep_schedule_id
WHERE
	person_sleep_schedule.person_id = $1
LIMIT 
   1
;";

pub async fn get_person_sleep_schedule_query(
    database_connection: &PgPool,
    person_id: &Uuid,
) -> Result<Option<SleepSchedule>> {
    info!("QUERY CALL: get_person_sleep_schedule_query");
    let query_result: Option<SleepSchedule> = query_as::<_, SleepSchedule>(QUERY)
        .bind(&person_id)
        .fetch_optional(database_connection)
        .await?;
    Ok(query_result)
}
