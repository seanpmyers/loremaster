use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Frequency {
    pub id: Uuid,
    pub unit: FrequencyUnit,
}

// These are defined in SQL, this should match what is there
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum FrequencyUnit {
    Day,
    Month,
    Year,
    Hour,
    Minute,
}

impl fmt::Display for FrequencyUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
