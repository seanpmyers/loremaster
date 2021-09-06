use anyhow::{Context, Result};
use chrono::{Date, Local};
use mobc::Connection;
use mobc_postgres::PgConnectionManager;
use tokio_postgres::NoTls;
use uuid::Uuid;

use crate::data::entity::person::{Person};

const CREATE_PERSON_QUERY : &str = "
    INSERT INTO
        public.person (email_address, hashed_password, creation_date, alias)
    VALUES 
    ($1, $2,TO_DATE($3, 'YYYY-MM-DD'), null)
    RETURNING
        id
    ;";

pub async fn create_person_query(database_connection: &Connection<PgConnectionManager<NoTls>>, email_address: &String, hashed_password: &String, creation_date: &Date<Local>) -> Result<Person> {
    
    let query_result = database_connection
    .query_one(CREATE_PERSON_QUERY, &[&email_address, &hashed_password, &creation_date.to_string()])
    .await
    .context(format!("An error occurred while querying the database."))?;

    let new_person: Person = Person {
        id: query_result.get::<_, Uuid>("id"),
        email_address: email_address.to_owned(),
        creation_date: Local::today(),
        alias: None,
    };

    return Ok(new_person);
}