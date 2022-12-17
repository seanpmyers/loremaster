use anyhow::{anyhow, Result};
use log::info;
use sqlx::{query, PgPool};

use uuid::Uuid;

const QUERY: &str = "
	INSERT INTO
		public.person_goal (
            person_id
            , goal_id
		)
	VALUES 
        ($1, $2)
;";

pub async fn add_goal_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    goal_id: &Uuid,
) -> Result<()> {
    info!("QUERY CALL: add_goal_query");

    let updated_row_count: u64 = query(QUERY)
        .bind(&person_id)
        .bind(&goal_id)
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
