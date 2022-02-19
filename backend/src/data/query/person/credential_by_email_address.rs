use anyhow::{anyhow, Result};
use log::error;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::{
    data::entity::person::Credentials,
    utility::constants::database::{EMAIL_ADDRESS, ENCRYPTED_PASSWORD, ID},
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
    let query_result: Result<Option<tokio_postgres::Row>, tokio_postgres::Error> =
        database_connection
            .query_opt(QUERY, &[&email_address])
            .await;

    match query_result {
        Ok(row) => {
            if let Some(person) = row {
                let result: Credentials = Credentials {
                    id: person.get::<_, Uuid>(ID),
                    email_address: person.get::<_, String>(EMAIL_ADDRESS),
                    encrypted_password: person.get::<_, String>(ENCRYPTED_PASSWORD),
                };

                return Ok(Some(result));
            } else {
                return Ok(None);
            }
        }
        Err(error) => {
            error!("{}", error);
            return Err(anyhow!("Something went wrong creating the new person."));
        }
    }
}
