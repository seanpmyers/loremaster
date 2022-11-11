use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Person {
    pub id: Uuid,
    #[serde(rename(serialize = "emailAddress"))]
    pub email_address: String,
    #[serde(rename(serialize = "registrationDate"))]
    pub registration_date: OffsetDateTime,
    #[serde(rename(serialize = "encryptedPassword"))]
    pub encrypted_password: String,
    pub alias: Option<String>,
    #[serde(rename(serialize = "chronicleId"))]
    pub chronicle_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Credentials {
    pub id: Uuid,
    pub email_address: String,
    pub encrypted_password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct PersonMeta {
    pub id: Uuid,
    #[serde(rename(serialize = "emailAddress"))]
    pub email_address: String,
    #[serde(rename(serialize = "registrationDate"))]
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
}

// pub struct SessionKey (String);
