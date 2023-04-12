use anyhow::Result;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const QUERY: &str = "
   SELECT
        chronicle.id
        , chronicle.person_id
        , chronicle.date_recorded
        , chronicle.notes
        , chronicle.creation_time
   FROM
        public.chronicle
   WHERE
        chronicle.id = $1
   LIMIT 
        1
;";

pub async fn chronicle_by_id_query(
    database_connection: &PgPool,
    chronicle_id: &Uuid,
) -> Result<Option<Chronicle>> {
    let query_result: Option<Chronicle> = query_as::<_, Chronicle>(QUERY)
        .bind(chronicle_id)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
