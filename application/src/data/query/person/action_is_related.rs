use anyhow::Result;
use log::info;
use sqlx::{postgres::PgRow, query, PgPool};

use uuid::Uuid;

const QUERY: &str = "
	SELECT
		person_action.action_id
	FROM
		person_action
	WHERE
		person_action.person_id = $1
		AND person_action.action_id = $2
	LIMIT
		1
;";

pub async fn action_is_related_query(
    database_connection: &PgPool,
    person_id: &Uuid,
    action_id: &Uuid,
) -> Result<bool> {
    info!("QUERY CALL: action_is_related_query");

    let query_result: Option<PgRow> = query(QUERY)
        .bind(&person_id)
        .bind(&action_id)
        .fetch_optional(database_connection)
        .await?;

    match query_result {
        Some(_relation) => Ok(true),
        None => Ok(false),
    }
}
