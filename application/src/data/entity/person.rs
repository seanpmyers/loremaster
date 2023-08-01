use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: Uuid,
    pub email_address_id: Uuid,
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
    pub chronicle_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub id: Uuid,
    pub email_address: String,
    pub encrypted_password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PersonMeta {
    pub id: Uuid,
    pub email_address: String,
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
}

// pub struct SessionKey (String);
