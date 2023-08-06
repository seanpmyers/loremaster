use anyhow::{anyhow, Result};
use log::info;
use sqlx::{query, PgPool};

use uuid::Uuid;

const QUERY: &str = "
	DELETE FROM
		public.person_goal
	WHERE
		person_goal.person_id = $1
		AND person_goal.goal_id = $2
;";

pub async fn remove_one_goal_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    goal_id: &Uuid,
) -> Result<()> {
    info!("QUERY CALL: remove_one_goal_query");

    let updated_row_count: u64 = query(QUERY)
        .bind(person_id)
        .bind(goal_id)
        .execute(database_connection)
        .await?
        .rows_affected();

    if updated_row_count < 1_u64 {
        return Err(anyhow!(
            "No rows were updated! query: remove_one_goal_query"
        ));
    }

    Ok(())
}
