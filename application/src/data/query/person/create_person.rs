use anyhow::{anyhow, Result};
use chrono::{offset, DateTime, Utc};
use log::error;
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Row, Statement};
use uuid::Uuid;

use crate::data::entity::person::Person;

const QUERY: &str = "
    INSERT INTO
        public.person (
            id,
            email_address, 
            encrypted_password,
            registration_date, 
            alias,
            chronicle_id
        )
    VALUES 
    ($1, $2, $3, $4, null, null)
    RETURNING
        id
    ;";

pub async fn create_person_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    email_address: &String,
    encrypted_password: &String,
) -> Result<Person> {
    let new_person_id: Uuid = Uuid::new_v4();
    let creation_date: DateTime<Utc> = Utc::now();

    let prepared_statement: Statement = database_connection.prepare(QUERY).await?;

    let query_result: Result<Row, tokio_postgres::Error> = database_connection
        .query_one(
            &prepared_statement,
            &[
                &new_person_id,
                &email_address,
                &encrypted_password,
                &creation_date,
            ],
        )
        .await;

    match query_result {
        Ok(_) => {
            let new_person: Person = Person {
                id: new_person_id.to_owned(),
                email_address: email_address.to_owned(),
                registration_date: offset::Utc::now(),
                alias: None,
            };

            return Ok(new_person);
        }
        Err(error) => {
            error!("{}", error);
            return Err(anyhow!("Something went wrong creating the new person."));
        }
    }
}
