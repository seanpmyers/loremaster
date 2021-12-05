use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Person {
    pub id : Uuid,
    pub email_address : String,
    pub creation_date: DateTime<Utc>,
    pub alias : Option<String>,
}

// pub struct Credentials {
//     pub email_address: String,
//     pub password: String
// }

// pub struct SessionKey (String);
