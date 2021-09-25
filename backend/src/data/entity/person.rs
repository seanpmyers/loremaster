use chrono::{Date, Local};
use uuid::Uuid;

#[derive(Debug)]
pub struct Person {
    pub id : Uuid,
    pub email_address : String,
    pub creation_date: Date<Local>,
    pub alias : Option<String>,
}

pub struct Credentials {
    pub email_address: String,
    pub password: String
}