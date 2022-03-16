use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Statement};
use uuid::Uuid;

use crate::{
    data::entity::person::Person,
    utility::constants::database::{EMAIL_ADDRESS, ID, _REGISTRATION_DATE},
};

const _QUERY: &str = "
SELECT
   email_address
   , creation_date
FROM
   public.person
WHERE
   person.email_address = $1
LIMIT 
   1
;";

pub async fn _person_by_email_address_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    email_address: &str,
) -> Result<Option<Person>> {
    let prepared_statement: Statement = database_connection.prepare(_QUERY).await?;

    let query_result: Option<tokio_postgres::Row> = database_connection
        .query_opt(&prepared_statement, &[&email_address])
        .await
        .map_err(|error| anyhow!("{}", error))?;

    if let Some(person) = query_result {
        let result: Person = Person {
            id: person.get::<_, Uuid>(ID),
            email_address: person.get::<_, String>(EMAIL_ADDRESS),
            registration_date: person.get::<_, DateTime<Utc>>(_REGISTRATION_DATE),
            alias: None,
        };

        Ok(Some(result))
    } else {
        Ok(None)
    }
}
