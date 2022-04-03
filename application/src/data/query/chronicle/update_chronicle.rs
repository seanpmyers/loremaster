use anyhow::Result;
use sqlx::{query_as, PgPool};

use crate::data::entity::chronicle::Chronicle;

const QUERY: &str = "
    UPDATE
        public.chronicle
    SET 
      date_recorded = (TO_DATE($1, 'YYYY-MM-DD'))
    WHERE
      id = $2
   RETURNING
      id
      , date_recorded
    ;";

pub async fn update_chronicle_query(
    database_connection: &PgPool,
    chronicle_to_update: &Chronicle,
) -> Result<Chronicle> {
    let query_result = query_as::<_, Chronicle>(QUERY)
        .bind(&chronicle_to_update.date_recorded)
        .bind(&chronicle_to_update.id)
        .fetch_one(database_connection)
        .await?;
    Ok(query_result)
}
