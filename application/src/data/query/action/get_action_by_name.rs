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
		WHERE
			name = $1
    ;";

pub async fn get_action_by_name_query(
    database_connection: &PgPool,
    action: &String,
) -> Result<Option<Action>> {
    info!("QUERY CALL: get_action_by_name_query");

    let query_result: Option<Action> = query_as::<_, Action>(QUERY)
        .bind(&action)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
