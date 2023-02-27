use anyhow::Result;

use crate::data::entity::frequency::Frequency;

pub fn get_frequency_list_query() -> Result<Vec<Frequency>> {
    Ok(vec![
        Frequency::Day,
        Frequency::Weekday,
        Frequency::Week,
        Frequency::Month,
        Frequency::Year,
        Frequency::Hour,
        Frequency::Minute,
    ])
}
