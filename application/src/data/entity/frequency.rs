use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Frequency {
    pub id: Uuid,
    pub unit: FrequencyUnit,
}

// These are defined in SQL, this should match what is there
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FrequencyUnit {
    Day,
    Month,
    Year,
    Hour,
    Minute,
}
