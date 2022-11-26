use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub id: Uuid,
    pub name: String,
}
