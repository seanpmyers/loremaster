use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EmailAddress {
    pub id: EmailAddressId,
    pub display: String,
    pub local_part: String,
    pub domain: String,
    pub validated: bool,
    pub validation_date: Option<OffsetDateTime>,
    pub creation_date: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct EmailAddressId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for EmailAddressId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
