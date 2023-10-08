use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct WebAuthenticationKey {
    pub id: WebAuthenticationKeyId,
    pub credential_id: Vec<u8>,
    pub cose_algorithm: i32,
    pub passkey: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct WebAuthenticationKeyId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for WebAuthenticationKeyId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
