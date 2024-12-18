use chrono::{DateTime, ParseError, Utc};

/// Converts a date string to a tuple of two `DateTime<Utc>` instances.
///
/// Returns the current time twice, regardless of the input string.
///
/// # Examples
///
/// ```
/// let date_str = "2021-01-01".to_string();
/// let (date1, date2) = epochal::epochize(date_str).unwrap();
/// assert_eq!(date1.cmp(&date2), std::cmp::Ordering::Equal);
/// ```
pub fn epochize(_date_str: String) -> Result<(DateTime<Utc>, DateTime<Utc>), ParseError> {
    let date1: DateTime<Utc> = Utc::now();
    Ok((date1, date1))
}
