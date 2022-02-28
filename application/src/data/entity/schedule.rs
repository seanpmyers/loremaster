// use chrono::{Date, DateTime, Utc, Weekday};
// use std::collections::HashSet;
// use uuid::Uuid;

// pub struct Schedule {
//    pub id: Uuid,
//    pub start_date_time: DateTime<Utc>,
//    pub end_date_time: Option<DateTime<Utc>>,
//    pub weekday_occurences: HashSet<Weekday>,
//    pub date_interval: TimeInterval,
//    pub time_interval: TimeInterval,
//    pub excluded_dates: Option<Vec<Date<Utc>>>,
// }

// pub struct TimeInterval {
//    pub value: u32,
//    pub time_unit: UnitOfTime,
// }

// pub enum UnitOfTime {
//    Second,
//    Minute,
//    Hour,
// }

// pub enum UnitOfMonth {
//    Day,
//    Week,
//    Month,
//    Year,
//    Decade,
//    Century
// }

