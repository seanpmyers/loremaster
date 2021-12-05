// use anyhow::{Result, anyhow};
// use chrono::{DateTime, Utc};
// use crate::data::entity::schedule::{Schedule};


// pub fn CalculateNextRunDate(schedule: Schedule) -> Result<Option<DateTime<Utc>>> {
//    // If the start date is in the future, it is the next run date
//    if schedule.start_date_time < Utc::now() { 
//       if let Some(excluded_dates) = schedule.excluded_dates {
//          if excluded_dates.contains(&schedule.start_date_time.date()) { return Err(anyhow!(""))}
//       }
//       return Ok(Some(schedule.start_date_time.clone()));
//    }

//    return Ok(Some(Utc::now()));
// }



