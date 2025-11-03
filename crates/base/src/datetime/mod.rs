pub mod unix;

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use std::time::Duration;

use crate::{Error, Result};

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M";
pub const DATE_TIME_LONG_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DATE_TIME_FULL_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";
pub const DATE_TIME_FULL_FORMAT_TZ: &str = "%Y-%m-%dT%H:%M:%S%.fZ";

pub fn parse_date_any(value: &str) -> Result<DateTime<Utc>> {
    let date = NaiveDateTime::parse_from_str(value, DATE_TIME_FORMAT)
        .or_else(|_| NaiveDateTime::parse_from_str(value, DATE_TIME_LONG_FORMAT))
        .or_else(|_| NaiveDateTime::parse_from_str(value, DATE_TIME_FULL_FORMAT))
        .or_else(|_| NaiveDateTime::parse_from_str(value, DATE_TIME_FULL_FORMAT_TZ))
        .or_else(|_| {
            NaiveDate::parse_from_str(value, DATE_FORMAT)
                .map(|d| NaiveDateTime::new(d, NaiveTime::MIN))
        })
        .map_err(Error::from_std_error)?;
    Ok(Utc.from_utc_datetime(&date))
}

pub fn parse_date(value: &str) -> Result<DateTime<Utc>> {
    let date = NaiveDateTime::parse_from_str(value, DATE_TIME_LONG_FORMAT)
        .map_err(Error::from_std_error)?;
    Ok(Utc.from_utc_datetime(&date))
}

pub fn parse_date_ftz(value: &str) -> Result<DateTime<Utc>> {
    let date = NaiveDateTime::parse_from_str(value, DATE_TIME_FULL_FORMAT_TZ)
        .map_err(Error::from_std_error)?;
    Ok(Utc.from_utc_datetime(&date))
}

pub fn utc_today() -> DateTime<Utc> {
    Utc.from_utc_datetime(&NaiveDateTime::new(Utc::now().date_naive(), NaiveTime::MIN))
}

pub fn format_duration(duration: Duration) -> String {
    format_seconds_long(duration.as_micros())
}

pub fn format_seconds(time: i64) -> String {
    Local
        .timestamp_opt(time, 0)
        .single()
        .map(|dt| dt.format("%H:%M:%S").to_string())
        .unwrap_or_else(|| "invalid time".to_string())
}

pub fn format_seconds_long(time: u128) -> String {
    let secs = (time / 1_000_000) as i64;
    let micros_in_sec = (time % 1_000_000) as u32;
    let nanos = micros_in_sec * 1_000;
    Local
        .timestamp_opt(secs, nanos)
        .single()
        .map(|dt| dt.format("%H:%M:%S%.6f").to_string())
        .unwrap_or_else(|| "invalid time".to_string())
}
