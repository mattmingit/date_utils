use date_utils::{
    datetime_to_date, parse_response_string_to_datetime, parse_to_datetime, timestamp_to_datetime,
    DateTimeError, DateType, OffsetType,
};
use time::macros::{date, datetime};

#[test]
fn test_parse_to_datetime() {
    assert_eq!(
        parse_to_datetime("2025-05-10", DateType::Start, OffsetType::Utc).unwrap(),
        datetime!(2025-05-10 0:00:00 UTC)
    );
    assert_eq!(
        parse_to_datetime("2025-05-09", DateType::End, OffsetType::Utc).unwrap(),
        datetime!(2025-05-09 23:59:59.999999999 UTC)
    );
}

#[test]
fn test_parse_to_datetime_error() {
    assert!(matches!(
        parse_to_datetime("2025/05/10", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::InvalidDateFormat(_, _)
    ));
    assert!(matches!(
        parse_to_datetime("202-05-10", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::InvalidDateFormat(_, _)
    ));
    assert!(matches!(
        parse_to_datetime("2025-14-10", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::InvalidDateFormat(_, _)
    ));
    assert!(matches!(
        parse_to_datetime("2025-14-40", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::InvalidDateFormat(_, _)
    ));
    assert!(matches!(
        parse_to_datetime("2025-05-40 12:", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::InvalidDateFormat(_, _)
    ));
    assert!(matches!(
        parse_to_datetime("2025-05-10 12:70", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::InvalidDateFormat(_, _)
    ));
    assert!(matches!(
        parse_to_datetime("2028-05-10", DateType::Start, OffsetType::Utc).unwrap_err(),
        DateTimeError::DateInFuture(_, _)
    ));
}

#[test]
fn test_timestamp_to_datetime() {
    let timestamp = 1732440896;
    let datetime = timestamp_to_datetime(timestamp, OffsetType::Utc).unwrap();
    assert_eq!(datetime.year(), 2024);
    assert_eq!(datetime.month() as u8, 11);
    assert_eq!(datetime.day(), 24);
    assert_eq!(datetime.hour(), 9);
    assert_eq!(datetime.minute(), 34);
    assert_eq!(datetime.second(), 56);
    assert_eq!(datetime.offset().whole_hours(), 0);
}

#[test]
fn test_datetime_to_date() {
    let datetime = datetime!(2025-05-09 23:59:59.999999999 UTC);
    let date = datetime_to_date(datetime).unwrap();
    assert_eq!(date, date!(2025 - 05 - 09));
}

#[test]
fn test_parse_response_string_to_datetime() {
    let date = parse_response_string_to_datetime("2024-05-31", OffsetType::Utc).unwrap();
    assert_eq!(date.date(), datetime!(2024-05-31 00:00 UTC).date());

    let date = parse_response_string_to_datetime("2024-05", OffsetType::Utc).unwrap();
    assert_eq!(date.date().month(), time::Month::May);

    let date = parse_response_string_to_datetime("2024-Q2", OffsetType::Utc).unwrap();
    assert_eq!(date.date().month(), time::Month::April); // Q2 starts in April
}
