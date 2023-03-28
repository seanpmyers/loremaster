use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::data::entity::quick_task::PersonQuickTask;

const QUERY: &str = "
	SELECT
		person_quick_task.quick_task_id
		, person_quick_task.person_id
		, quick_task.title
		, quick_task.description
		, quick_task.completed
	FROM
		public.person_quick_task
	INNER JOIN
		public.quick_task
	WHERE
		person_quick_task.person_id = $1
	;";

pub async fn get_quick_task_list_by_person_id(
    database_connection: &PgPool,
    person_id: &Uuid,
) -> Result<Vec<PersonQuickTask>> {
    info!("QUERY CALL: get_quick_task_list_by_person_id");

    Ok(query_as::<_, PersonQuickTask>(QUERY)
        .bind(person_id)
        .fetch_all(database_connection)
        .await?)
}
