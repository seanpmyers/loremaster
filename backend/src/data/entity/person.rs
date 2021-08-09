use chrono::{Date, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Person {
    pub id : Uuid,
    pub email_address : String,
    pub creation_date: Date<Utc>,
    pub alias : Option<String>,
}

pub struct Credentials {
    pub id: Uuid,
    pub email_address: String,
    pub password: String
}