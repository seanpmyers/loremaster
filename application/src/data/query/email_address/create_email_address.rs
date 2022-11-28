use anyhow::Result;

use log::info;
use sqlx::{query_as, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::data::entity::email_address::EmailAddress;

const EXISTING_ADDRESS_QUERY: &str = "
    SELECT
        id
        , display 
        , local_part
        , domain 
        , validated
        , validation_date
        , creation_date
    FROM
        public.email_address
    WHERE
        email_address.display = $1
    LIMIT
    1
;";

const QUERY: &str = "
    INSERT INTO
        public.email_address (
            id
            , display 
            , local_part
            , domain 
            , creation_date
        )
    VALUES 
    ($1, $2, $3, $4, $5)
    RETURNING
			id
			, display 
			, local_part
			, domain 
			, validated
            , validation_date
			, creation_date
    ;";

pub async fn create_email_address_query(
    database_connection: &PgPool,
    email_address: &email_address::EmailAddress,
) -> Result<EmailAddress> {
    info!("QUERY CALL: create_person_query");
    let new_id: Uuid = Uuid::new_v4();
    let creation_date: OffsetDateTime = OffsetDateTime::now_utc();

    let potential_address: Option<EmailAddress> =
        query_as::<_, EmailAddress>(EXISTING_ADDRESS_QUERY)
            .bind(&email_address.as_str())
            .fetch_optional(database_connection)
            .await?;

    if let Some(existing_email_address) = potential_address {
        return Ok(existing_email_address);
    }

    let query_result: EmailAddress = query_as::<_, EmailAddress>(QUERY)
        .bind(&new_id)
        .bind(&email_address.as_str())
        .bind(&email_address.local_part())
        .bind(&email_address.domain())
        .bind(&creation_date)
        .fetch_one(database_connection)
        .await?;

    Ok(query_result)
}
