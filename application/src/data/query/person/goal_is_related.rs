use anyhow::Result;
use log::info;
use sqlx::{query, PgPool};

use uuid::Uuid;

const QUERY: &str = "
	SELECT
		person_goal.action_id
	FROM
		person_goal
	WHERE
        person_goal.person_id = $1
        AND person_goal.goal_id = $2
	LIMIT
		1
;";

pub async fn goal_is_related_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    goal_id: &Uuid,
) -> Result<bool> {
    info!("QUERY CALL: goal_is_related_query");

    let query_result = query(QUERY)
        .bind(&person_id)
        .bind(&goal_id)
        .fetch_optional(database_connection)
        .await?;

    match query_result {
        Some(_relation) => Ok(true),
        None => Ok(false),
    }
}
