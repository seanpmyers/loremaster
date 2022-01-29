use anyhow::{
   Context, 
   Result
};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::person::{
   Credentials
};

const QUERY: &str = "
   SELECT
      id
      , email_address
      , encrypted_password
   FROM
      public.person
   WHERE
      person.email_address = $1
   LIMIT 
      1
;";

pub async fn credential_by_email_address_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>, 
    email_address: &String, 
) -> Result<Option<Credentials>> {
    
    let query_result = database_connection
        .query_opt(
            QUERY, 
            &[
                &email_address
                ]
            )
        .await
        .context("An error occurred while querying the database.".to_string())?
    ;

    if let Some(person) = query_result {
      let result: Credentials = Credentials {
         id: person.get::<_, Uuid>("id"),
         email_address: person.get::<_, String>("email_address"),
         encrypted_password: person.get::<_, String>("encrypted_password")
       };
   
       return Ok(Some(result));
    }
    else { return Ok(None); }
}