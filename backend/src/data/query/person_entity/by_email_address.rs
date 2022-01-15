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

const QUERY
 : &str = "
SELECT
   email_address
   , hashed_password
   , creation_date
FROM
   public.loremaster.person
WHERE
   person.email_address = '$1'
LIMIT 
   1
;";

pub async fn by_email_address(
    database_connection: &Connection<PgConnectionManager<NoTls>>, 
    email_address: &String, 
) -> Result<Person> {
    
    let query_result = database_connection
        .query_one(
         QUERY, 
            &[&email_address])
        .await
        .context(format!("An error occurred while querying the database."))?
    ;

    let new_person: Person = Person {
        id: query_result.get::<_, Uuid>("id"),
        email_address: query_result.get::<_, String>("email_address"),
        creation_date: query_result.get::<_, DateTime<Utc>>("creation_date"),
        alias: None,
    };

    return Ok(new_person);
}