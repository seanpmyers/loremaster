use anyhow::Result;
use email_address::EmailAddress;
use serde_json::Value;
use sqlx::{query_scalar, PgPool};
use webauthn_rs::prelude::Passkey;

const QUERY: &str = "
	SELECT 
		passkey
	FROM
		email_address 
	INNER JOIN person ON person.email_address_id  = email_address .id 
	INNER JOIN person_web_authentication_key ON person.id = person_web_authentication_key.person_id 
	INNER JOIN web_authentication_key ON person_web_authentication_key.web_authentication_key_id = web_authentication_key.id
	WHERE
		email_address.local_part = $1
		AND email_address.domain = $2
;";

pub async fn get_webauthn_passkeys_by_email_address_query(
    database_connection: &PgPool,
    email_address: &EmailAddress,
) -> Result<Vec<Passkey>> {
    let query_result: Vec<Value> = query_scalar(QUERY)
        .bind(email_address.local_part())
        .bind(email_address.domain())
        .fetch_all(database_connection)
        .await?;

    Ok(query_result
        .into_iter()
        .map(|json| {
            let key: Passkey = serde_json::from_value(json).unwrap();
            key
        })
        .collect::<Vec<Passkey>>())
}
