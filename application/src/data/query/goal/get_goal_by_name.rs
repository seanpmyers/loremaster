use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};

use crate::data::entity::goal::Goal;

const QUERY: &str = "
    SELECT
			id
			, name
		FROM
			public.goal
		WHERE
			name = $1
    ;";

pub async fn get_goal_by_name_query(
    database_connection: &PgPool,
    goal: &String,
) -> Result<Option<Goal>> {
    info!("QUERY CALL: get_goal_by_name_query");

    let query_result: Option<Goal> = query_as::<_, Goal>(QUERY)
        .bind(&goal)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
