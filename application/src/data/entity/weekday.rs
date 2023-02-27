use serde::{Deserialize, Serialize};

// These are defined in SQL, this should match what is there
#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "weekday")]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
