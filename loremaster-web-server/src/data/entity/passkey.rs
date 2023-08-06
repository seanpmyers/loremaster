use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Passkey {
    pub id: Uuid,
    pub credential_id: Vec<u8>,
    pub cose_algorithm: i32,
}
