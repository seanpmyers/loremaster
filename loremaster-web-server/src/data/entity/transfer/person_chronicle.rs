use serde::{Deserialize, Serialize};
use time::Date;

use crate::data::entity::chronicle::ChronicleId;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersonChronicle {
    pub chronicle_id: ChronicleId,
    pub chronicle_date: Date,
    pub person_alias: Option<String>,
}
