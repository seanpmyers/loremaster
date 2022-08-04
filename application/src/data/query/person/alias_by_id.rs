use anyhow::Result;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct PersonAlias {
    pub alias: Option<String>,
}

const QUERY: &str = "
SELECT
   person.alias
FROM
   public.person
WHERE
   person.id = $1
LIMIT 
   1
;";

pub async fn alias_by_id_query(
    database_connection: &PgPool,
    person_id: &Uuid,
) -> Result<Option<String>> {
    let query_result: Option<PersonAlias> = query_as::<_, PersonAlias>(QUERY)
        .bind(&person_id)
        .fetch_optional(database_connection)
        .await?;
    match query_result {
        Some(person) => Ok(person.alias),
        None => Ok(None),
    }
}
