use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Date, Duration, OffsetDateTime};
use time_tz::OffsetDateTimeExt;
use uuid::Uuid;

use super::person::PersonId;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Chronicle {
    pub id: ChronicleId,
    pub person_id: PersonId,
    pub date_recorded: Date,
    pub notes: Option<String>,
    pub creation_time: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::Decode, sqlx::Encode)]
pub struct ChronicleId(pub Uuid);

impl sqlx::Type<sqlx::Postgres> for ChronicleId {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Uuid as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

pub fn current_server_time() -> Result<OffsetDateTime> {
    Ok(OffsetDateTime::now_utc())
}

pub fn current_sever_time_string() -> Result<String> {
    Ok(OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| anyhow!("{}", error))?)
}

pub fn get_date_from_timezone(
    date_time: OffsetDateTime,
    timezone_string: &str,
) -> Result<OffsetDateTime> {
    match time_tz::timezones::get_by_name(timezone_string) {
        Some(timezone) => Ok(date_time.to_timezone(timezone)),
        None => Ok(date_time),
    }
}

pub fn _is_overdue(
    start_date: OffsetDateTime,
    duration: Duration,
    end_date: Option<OffsetDateTime>,
) -> Result<bool> {
    match end_date {
        Some(end_date) => Ok(start_date.checked_add(duration).unwrap() < end_date),
        None => Ok(start_date.checked_add(duration).unwrap() > current_server_time()?),
    }
}

pub fn _calculate_next_occurrence(
    start_date: OffsetDateTime,
    duration: Duration,
) -> Result<OffsetDateTime> {
    Ok(start_date.checked_add(duration).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::data::entity::chronicle::{_calculate_next_occurrence, _is_overdue};

    use super::{current_server_time, get_date_from_timezone};
    use anyhow::Result;
    use time::{
        macros::{datetime, offset},
        OffsetDateTime,
    };

    const NEW_YORK_TIME_ZONE_STRING: &str = "America/New_York";
    const MELBOURNE_TIME_ZONE_STRING: &str = "Australia/Melbourne";

    #[test]
    fn verify_date_from_timezone_new_york() -> Result<()> {
        let expected_result: time::Date = OffsetDateTime::now_utc().to_offset(offset!(-5)).date();
        let result: time::Date =
            get_date_from_timezone(current_server_time()?, NEW_YORK_TIME_ZONE_STRING)?.date();
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn verify_date_from_timezone_melbourne() -> Result<()> {
        let expected_result: time::Date = OffsetDateTime::now_utc().to_offset(offset!(+11)).date();
        let result: time::Date =
            get_date_from_timezone(current_server_time()?, MELBOURNE_TIME_ZONE_STRING)?.date();
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn verify_date_from_timezone_bad_input() -> Result<()> {
        let expected_result: time::Date = OffsetDateTime::now_utc().date();
        let result: time::Date = get_date_from_timezone(current_server_time()?, "bad")?.date();
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn verify_calculate_next_occurrence() -> Result<()> {
        let expected_result: OffsetDateTime = datetime!(2023-01-02 0:00 UTC);
        let start_date: OffsetDateTime = datetime!(2023-01-01 0:00 UTC);
        let duration: time::Duration = time::Duration::days(1);
        let result: OffsetDateTime = _calculate_next_occurrence(start_date, duration)?;
        assert_eq!(result, expected_result);
        Ok(())
    }

    #[test]
    fn verify_overdue_true() -> Result<()> {
        const EXPECTED_RESULT: bool = true;
        let end_date: OffsetDateTime = datetime!(2023-01-03 0:00 UTC);
        let start_date: OffsetDateTime = datetime!(2023-01-01 0:00 UTC);
        let duration: time::Duration = time::Duration::days(1);
        let result: bool = _is_overdue(start_date, duration, Some(end_date))?;
        assert_eq!(result, EXPECTED_RESULT);
        Ok(())
    }

    #[test]
    fn verify_overdue_false() -> Result<()> {
        const EXPECTED_RESULT: bool = false;
        let end_date: OffsetDateTime = datetime!(2023-01-02 0:00 UTC);
        let start_date: OffsetDateTime = datetime!(2023-01-01 0:00 UTC);
        let duration: time::Duration = time::Duration::days(1);
        let result: bool = _is_overdue(start_date, duration, Some(end_date))?;
        assert_eq!(result, EXPECTED_RESULT);
        Ok(())
    }
}
