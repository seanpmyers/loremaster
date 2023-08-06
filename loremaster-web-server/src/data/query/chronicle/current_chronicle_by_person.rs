use anyhow::Result;
use sqlx::{query_as, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::data::entity::chronicle::Chronicle;

const QUERY: &str = "
    SELECT DISTINCT
        chronicle.id
        , chronicle.person_id
        , chronicle.date_recorded
        , chronicle.notes
        , chronicle.creation_time
    FROM
        public.chronicle
    WHERE
        chronicle.date_recorded = $1
        AND chronicle.person_id = $2
    LIMIT 1
    ;";

pub async fn get_current_chronicle_by_person_query(
    database_connection: &PgPool,
    date: &OffsetDateTime,
    person_id: &Uuid,
) -> Result<Option<Chronicle>> {
    let query_result: Option<Chronicle> = query_as::<_, Chronicle>(QUERY)
        .bind(date.date())
        .bind(person_id)
        .fetch_optional(database_connection)
        .await?;

    Ok(query_result)
}
