use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Frequency {
    pub id: Uuid,
    pub unit: FrequencyUnit,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FrequencyUnit {
    Day,
    Month,
    Year,
    Hour,
    Minute,
}
