use anyhow::Result;
use log::info;
use sqlx::{query_as, PgPool};

use crate::data::entity::frequency::Frequency;

const QUERY: &str = "
    SELECT
			id
			, unit
		FROM
			public.frequency
    ;";

pub async fn get_frequency_list_query(database_connection: &PgPool) -> Result<Vec<Frequency>> {
    info!("QUERY CALL: get_frequency_list_query");

    let query_result: Vec<Frequency> = query_as::<_, Frequency>(QUERY)
        .fetch_all(database_connection)
        .await?;

    Ok(query_result)
}
