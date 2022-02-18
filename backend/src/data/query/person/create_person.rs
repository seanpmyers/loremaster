use anyhow::{Context, Result};
use chrono::{offset, DateTime, Utc};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::{NoTls, Row};
use uuid::Uuid;

use crate::data::entity::person::Person;

const QUERY: &str = "
    INSERT INTO
        public.person (
            email_address, 
            encrypted_password,
            registration_date, 
            alias,
            chronicle_id
        )
    VALUES 
    ($1, $2, TO_DATE($3, 'YYYY-MM-DD'), null, null)
    RETURNING
        id
    ;";

pub async fn create_person_query(
    database_connection: &Connection<PgConnectionManager<NoTls>>,
    email_address: &String,
    encrypted_password: &String,
) -> Result<Person> {
    let creation_date: DateTime<Utc> = Utc::now();
    let query_result: Row = database_connection
        .query_one(
            QUERY,
            &[
                &email_address,
                &encrypted_password,
                &creation_date.to_string(),
            ],
        )
        .await
        .context("An error occurred while querying the database.".to_string())?;

    let new_person: Person = Person {
        id: query_result.get::<_, Uuid>("id"),
        email_address: email_address.to_owned(),
        creation_date: offset::Utc::now(),
        alias: None,
    };

    return Ok(new_person);
}
