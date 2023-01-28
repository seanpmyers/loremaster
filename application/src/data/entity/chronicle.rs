use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, Date, OffsetDateTime};
use time_tz::OffsetDateTimeExt;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Chronicle {
    pub id: Uuid,
    pub person_id: Uuid,
    pub date_recorded: Date,
    pub notes: Option<String>,
    pub creation_time: Option<OffsetDateTime>,
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
    match time_tz::timezones::get_by_name(&timezone_string) {
        Some(timezone) => Ok(date_time.to_timezone(timezone)),
        None => Ok(date_time),
    }
}

#[cfg(test)]
mod tests {
    use super::{current_server_time, get_date_from_timezone};
    use anyhow::Result;
    use time::{macros::offset, OffsetDateTime};

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
}
