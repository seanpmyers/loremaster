use anyhow::{
   Context, 
   Result
};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::person::{
   PersonCredentials
};

const QUERY
 : &str = "
SELECT
   id
   , email_address
   , hashed_password
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
) -> Result<PersonCredentials> {
    
    let query_result = database_connection
        .query_one(
            QUERY, 
            &[
                &email_address, 
                ]
            )
        .await
        .context(format!("An error occurred while querying the database."))?
    ;

    let result: PersonCredentials = PersonCredentials {
      id: query_result.get::<_, Uuid>("id"),
      email_address: query_result.get::<_, String>("email_address"),
      encrypted_password: query_result.get::<_, String>("encrypted_password")
    };

    return Ok(result);
}