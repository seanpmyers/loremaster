use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ChronicleIntention {
    pub intention_id: Uuid,
    pub chronicle_id: Uuid,
    pub person_id: Uuid,
    pub action_id: Uuid,
    pub action: String,
}
