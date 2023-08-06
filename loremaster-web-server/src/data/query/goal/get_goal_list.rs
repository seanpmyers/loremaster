use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::data::entity::goal::Goal;

const QUERY: &str = "
	SELECT
		id
		, name
	FROM
		public.goal
	;";

const PERSON_QUERY: &str = "
	SELECT
		goal.id
		, goal.name
	FROM
		public.goal
	INNER JOIN
		public.person_goal ON goal.id = person_goal.goal_id
	WHERE
		person_goal.person_id = $1
";

pub async fn get_goal_list_query(
    database_connection: &PgPool,
    person_id: Option<&Uuid>,
) -> Result<Vec<Goal>> {
    info!("QUERY CALL: get_goal_list_query");

    match person_id {
        Some(id) => Ok(query_as::<_, Goal>(PERSON_QUERY)
            .bind(id)
            .fetch_all(database_connection)
            .await?),
        None => Ok(query_as::<_, Goal>(QUERY)
            .fetch_all(database_connection)
            .await?),
    }
}
