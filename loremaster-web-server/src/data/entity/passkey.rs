use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Passkey {
    pub id: PasskeyId,
    pub credential_id: Vec<u8>,
    pub cose_algorithm: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct PasskeyId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for PasskeyId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
