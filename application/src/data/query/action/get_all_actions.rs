use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};

use crate::data::entity::action::Action;

const QUERY: &str = "
    SELECT
			id
			, name
		FROM
			public.action
    ;";

pub async fn get_all_actions_query(database_connection: &PgPool) -> Result<Vec<Action>> {
    info!("QUERY CALL: get_action_query");

    let query_result: Vec<Action> = query_as::<_, Action>(QUERY)
        .fetch_all(database_connection)
        .await?;

    Ok(query_result)
}
