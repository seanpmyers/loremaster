use anyhow::Result;
use sqlx::{query_as, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const QUERY: &str = "
   SELECT
      chronicle.id
      , chronicle.person_id
      , chronicle.date_recorded
   FROM
      public.chronicle
   WHERE
      chronicle.date_recorded = $1
      AND chronicle.person_id = $2
   LIMIT 1
    ;";

pub async fn chronicle_by_date_query(
    database_connection: &PgPool,
    chronicle_date: &OffsetDateTime,
    person_id: &Uuid,
) -> Result<Option<Chronicle>> {
    let query_result = query_as::<_, Chronicle>(QUERY)
        .bind(chronicle_date)
        .bind(person_id)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
