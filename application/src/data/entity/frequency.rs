use serde::{Deserialize, Serialize};

// These are defined in SQL, this should match what is there
#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "frequency_unit")]
pub enum Frequency {
    Day,
    Weekday,
    Week,
    Month,
    Year,
    Hour,
    Minute,
}
