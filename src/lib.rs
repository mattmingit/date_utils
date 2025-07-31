//! Date Handling Module
//!
//! This module contains a set of methods to handle date conversion in different
//! formats (Date, datetime, string, timestamp).
use thiserror::Error;
use time::{
    macros::format_description, Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset,
};

/// Represents error related to dates parsing.
#[derive(Debug, Error)]
pub enum DateTimeError {
    #[error("Failed to parse date '{0}': {1}.")]
    InvalidDateFormat(String, String),
    #[error("Failed to convert timestamp '{0}' into datetime: {1}.")]
    InvalidTimestamp(i64, String),
    #[error("Invalid datetime format. Expected %Y-%m-%d %H:%M:%S, but got '{0}'.")]
    InvalidDateTimeFormat(String),
    #[error("Invalid time component: {0}.")]
    InvalidTimeComponent(String),
    #[error("Provided date '{0}' is in the future: '{0}' > '{1}'.")]
    DateInFuture(String, String),
    #[error("Failed to convert offset timestamp '{0}' into offset: {1}")]
    InvalidOffset(i32, String),
    #[error("Parsing failed: {0}")]
    ParseError(String),
}

impl From<time::error::IndeterminateOffset> for DateTimeError {
    fn from(err: time::error::IndeterminateOffset) -> Self {
        DateTimeError::ParseError(err.to_string())
    }
}

/// Represents the date type.
///
/// - `DateType::Start`: It indicates the starting date
/// - `DateType::End`: It indicates the ending date
pub enum DateType {
    Start,
    End,
}

pub enum OffsetType {
    Local,
    Utc,
}

/// Converts the start or end date into datetime.
///
/// The function takes a date as argument and converts it into a datetime object.
/// It returns an OffsetDateTime object representing the date passed as argument. If the conversion fails
/// it returns a `DateTimeError`.
///
/// ## Returns
/// - `Ok(OffsetDateTime)`: An `OffsetDateTime` representing the date passed as argument.
/// - `Err(DateTimeError)`: If the conversion fails.
///
/// ## Example
/// ```rust,no_run
/// use date_utils::{parse_to_datetime, DateType, OffsetType};
///
/// let date = parse_to_datetime("2025-01-01", DateType::Start, OffsetType::Utc).unwrap();
/// println!("{}", date);
/// ```
pub fn parse_to_datetime(
    date: &str,
    date_type: DateType,
    offset_type: OffsetType,
) -> Result<OffsetDateTime, DateTimeError> {
    let date_fmt = format_description!("[year]-[month]-[day]");
    let date = Date::parse(date, date_fmt)
        .map_err(|err| DateTimeError::InvalidDateFormat(date.to_string(), err.to_string()))?;
    let primitive_datetime = match date_type {
        DateType::Start => PrimitiveDateTime::new(date, Time::MIDNIGHT),
        DateType::End => PrimitiveDateTime::new(date, Time::MAX),
    };
    let datetime_utc = primitive_datetime.assume_utc();
    let result = match offset_type {
        OffsetType::Utc => datetime_utc,
        OffsetType::Local => datetime_utc.to_offset(UtcOffset::local_offset_at(datetime_utc)?),
    };
    validate_not_in_future(result)?;
    Ok(result)
}

/// Validates that given `OffsetDateTime` is not in the future
///
/// The function checks whether the `OffsetDateTime` object is in future. It returns OK(()) if the datetime
/// is valid. If it is not valid, it returns a `DateTimeError`.
///
/// ## Returns
/// - `OK(())`: The `OffsetDateTime` is validated correctly and it is not in the future.
/// - `Err(DateTimeError)`: If the `OffsetDateTime` is in the future.
fn validate_not_in_future(datetime: OffsetDateTime) -> Result<(), DateTimeError> {
    let now = OffsetDateTime::now_utc();
    if datetime > now {
        return Err(DateTimeError::DateInFuture(
            datetime.to_string(),
            now.to_string(),
        ));
    }
    Ok(())
}

/// Converts the timestamp into datetime.
///
/// The function takes a timestamp as argument and converts it into a datetime object.
/// It returns an `OffsetDateTime` object representing the datetime object of the timestamp passed as
/// argument. If the conversion fails it returns a `DateTimeError`.
///
/// ## Returns
/// - `Ok(OffsetDateTime)`: An `OffsetDateTime` representing the timestamp passed as argument.
/// - `Err(DateTimeError)`: If the conversion fails.
///
/// ## Example
/// ```rust
/// use date_utils::{timestamp_to_datetime, OffsetType};
///
/// let datetime = timestamp_to_datetime(1732440896, OffsetType::Utc).unwrap();
/// println!("{}", datetime);
/// ```
pub fn timestamp_to_datetime(
    timestamp: i64,
    offset_type: OffsetType,
) -> Result<OffsetDateTime, DateTimeError> {
    let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
        .map_err(|err| DateTimeError::InvalidTimestamp(timestamp, err.to_string()))?
        .to_offset(UtcOffset::UTC);
    let result = match offset_type {
        OffsetType::Utc => datetime,
        OffsetType::Local => datetime.to_offset(UtcOffset::local_offset_at(datetime)?),
    };
    Ok(result)
}

/// Converts the datetime String into a simple date String.
///
/// The function takes a datetime String as argument and converts it into a simple date String.
/// It returns a `String` representing the date without time of the datetime passed as argument. If the
/// conversion fails it returns a `DateTimeError`.
///
/// ## Returns
/// - `Ok(String)`: A `String` representing the simple date.
/// - `Err(DateTimeError)`: If conversion fails.
///
/// ## Example
/// ```rust
/// use date_utils::datetime_to_date;
/// use time::macros::datetime;
///
/// let date = datetime_to_date(datetime!(2025-05-09 23:59:59.999999999 UTC)).unwrap();
/// println!("{}", date);
/// ```
pub fn datetime_to_date(date: OffsetDateTime) -> Result<Date, DateTimeError> {
    Ok(date.date())
}

/// Converts the offset (in timestamp notation) into offset.
///
/// The function takes an offset, expressed in seconds, as argument and converts it into an offset object.
/// It returns an `UtcOffset` object representing the converted offset. If the conversion fails it return a `DateTimeError`.
///
/// ## Arguments
/// - `offset_secs: i32`: The offset expressed in seconds.
///
/// ## Returns
/// - `Ok(UtcOffset)`: An offset object from UTC, can store values up to ±25:59:59.
/// - `Err(DateTimeError)`: If the conversion fails.
///
/// ## Example
/// ```rust
/// use date_utils::timestamp_to_offset;
///
/// let offset = timestamp_to_offset(-14400).unwrap();
/// println!("{}", offset);
/// ```
pub fn timestamp_to_offset(offset_secs: i32) -> Result<UtcOffset, DateTimeError> {
    UtcOffset::from_whole_seconds(offset_secs)
        .map_err(|err| DateTimeError::InvalidOffset(offset_secs, err.to_string()))
}

/// Parse a time period string into an `OffsetDateTime`.
///
/// This function supports multiple date formats commonly returned by SDMX APIs:
/// - **full date**: `"YYYY-MM-DD"` (e.g. `"2024-05-31`)
/// - **year-month**: `"YYYY-MM"` (e.g. `"2024-05"`), defaults to the first day of the month at midnight UTC.
/// - **quarterly**: `"YYYY-QN"` (e.g. `"2024-Q2"`), maps to the first day of the starting month of the quarter at midnight UTC.
///
/// ## Arguments
/// - `time_period`: A string slice representing a time period in one of the supported formats.
///
/// ## Returns
/// - `Ok(OffsetDateTime)`: The parsed datetime, assumed to be in UTC.
/// - `Err(DateTimeError)`: If the format is not recognized or parsing fails.
///
/// ## Example
/// ```rust
/// use date_utils::{OffsetType, parse_response_string_to_datetime };
/// use time::macros::datetime;
///
/// let date = parse_response_string_to_datetime("2024-05-31", OffsetType::Utc).unwrap();
/// assert_eq!(date.date(), datetime!(2024-05-31 00:00 UTC).date());
///
/// let date = parse_response_string_to_datetime("2024-05", OffsetType::Utc).unwrap();
/// assert_eq!(date.date().month(), time::Month::May);
///
/// let date = parse_response_string_to_datetime("2024-Q2", OffsetType::Utc).unwrap();
/// assert_eq!(date.date().month(), time::Month::April);  // Q2 starts in April
/// ```
pub fn parse_response_string_to_datetime(
    time_period: &str,
    offset_type: OffsetType,
) -> Result<OffsetDateTime, DateTimeError> {
    // Handle full date: YYYY-MM-DD
    if let Ok(date) = parse_to_datetime(time_period, DateType::End, offset_type) {
        return Ok(date);
    }

    // Handle year-month: YYYY-MM
    if let Some((year_str, month_str)) = time_period.split_once("-") {
        if let (Ok(year), Ok(month)) = (year_str.parse::<i32>(), month_str.parse::<u8>()) {
            let date = Date::from_calendar_date(
                year,
                Month::try_from(month).map_err(|err| {
                    DateTimeError::InvalidDateFormat(time_period.to_string(), err.to_string())
                })?,
                1,
            )
            .map_err(|err| DateTimeError::ParseError(err.to_string()))?;
            let datetime = PrimitiveDateTime::new(date, Time::MIDNIGHT).assume_utc();
            return Ok(datetime);
        }
    }

    // Handle quarterly format: "2023-Q1"
    if time_period.len() == 7 && &time_period[5..6] == "Q" {
        let year = time_period[0..4].parse::<i32>().map_err(|err| {
            DateTimeError::InvalidDateFormat(time_period.to_string(), err.to_string())
        })?;
        let quarter = time_period[6..7].parse::<u8>().map_err(|err| {
            DateTimeError::InvalidDateFormat(time_period.to_string(), err.to_string())
        })?;
        let month = match quarter {
            1 => Month::January,
            2 => Month::April,
            3 => Month::July,
            4 => Month::October,
            _ => return Err(DateTimeError::InvalidTimeComponent(quarter.to_string())),
        };
        let date = Date::from_calendar_date(year, month, 1)
            .map_err(|err| DateTimeError::ParseError(err.to_string()))?;
        let datetime = PrimitiveDateTime::new(date, Time::MIDNIGHT).assume_utc();
        return Ok(datetime);
    }
    Err(DateTimeError::ParseError(format!(
        "Unsupported date format: {time_period}"
    )))
}
