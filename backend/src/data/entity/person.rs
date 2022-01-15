use chrono::{
    DateTime, 
    Utc
};
use serde::{
    Deserialize, 
    Serialize
};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Person {
    pub id : Uuid,
    pub email_address : String,
    pub creation_date: DateTime<Utc>,
    pub alias : Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonCredentials {
    pub id: Uuid,
    pub email_address: String,
    pub encrypted_password: String
}

// pub struct SessionKey (String);
