use anyhow::{
    Context, 
    Result
};
use chrono::{
    DateTime, 
    Utc
};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::person::{Person};

const QUERY: &str = "
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

pub async fn person_by_email_address_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>, 
    email_address: &String, 
) -> Result<Option<Person>> {
    
    let query_result = database_connection
        .query_opt(
         QUERY, 
            &[&email_address])
        .await
        .context("An error occurred while querying the database.".to_string())?
    ;
    
    if let Some(person) = query_result {
        let result: Person = Person {
            id: person.get::<_, Uuid>("id"),
            email_address: person.get::<_, String>("email_address"),
            creation_date: person.get::<_, DateTime<Utc>>("creation_date"),
            alias: None,
        };
    
        return Ok(Some(result));
    }
    else { return Ok(None); }

    
}