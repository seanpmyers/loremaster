use std::fmt;

use serde::{Deserialize, Serialize};

// These are defined in SQL, this should match what is there
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Frequency {
    Day,
    Week,
    Weekday,
    Month,
    Year,
    Hour,
    Minute,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
