use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use super::email_address::EmailAddressId;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: PersonId,
    pub email_address_id: EmailAddressId,
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
    pub chronicle_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct PersonId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for PersonId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub id: PersonId,
    pub email_address: String,
    pub encrypted_password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PersonMeta {
    pub id: PersonId,
    pub email_address: String,
    pub registration_date: OffsetDateTime,
    pub alias: Option<String>,
}

// pub struct SessionKey (String);
