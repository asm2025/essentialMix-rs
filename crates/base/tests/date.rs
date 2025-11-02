#[cfg(test)]
mod tests {
    use chrono::{Utc, Datelike, Timelike};
    use emix::date;
    use std::time::Duration;

    #[test]
    fn test_parse_date() {
        let date_str = "2024-01-15 14:30:00";
        let result = date::parse_date(date_str);
        assert!(result.is_ok(), "Should parse date successfully");
        
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
    }

    #[test]
    fn test_parse_date_any_with_datetime_format() {
        let date_str = "2024-01-15 14:30";
        let result = date::parse_date_any(date_str);
        assert!(result.is_ok(), "Should parse date with datetime format");
    }

    #[test]
    fn test_parse_date_any_with_long_format() {
        let date_str = "2024-01-15 14:30:00";
        let result = date::parse_date_any(date_str);
        assert!(result.is_ok(), "Should parse date with long format");
    }

    #[test]
    fn test_parse_date_any_with_full_format() {
        let date_str = "2024-01-15 14:30:00.123456";
        let result = date::parse_date_any(date_str);
        assert!(result.is_ok(), "Should parse date with full format");
    }

    #[test]
    fn test_parse_date_any_with_tz_format() {
        let date_str = "2024-01-15T14:30:00.123456Z";
        let result = date::parse_date_any(date_str);
        assert!(result.is_ok(), "Should parse date with timezone format");
    }

    #[test]
    fn test_parse_date_any_with_date_only() {
        let date_str = "2024-01-15";
        let result = date::parse_date_any(date_str);
        assert!(result.is_ok(), "Should parse date only format");
        
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 15);
    }

    #[test]
    fn test_parse_date_ftz() {
        let date_str = "2024-01-15T14:30:00.123456Z";
        let result = date::parse_date_ftz(date_str);
        assert!(result.is_ok(), "Should parse full timezone format");
    }

    #[test]
    fn test_utc_today() {
        let today = date::utc_today();
        let now = Utc::now();
        
        assert_eq!(today.year(), now.year());
        assert_eq!(today.month(), now.month());
        assert_eq!(today.day(), now.day());
        assert_eq!(today.hour(), 0);
        assert_eq!(today.minute(), 0);
        assert_eq!(today.second(), 0);
    }

    #[test]
    fn test_format_duration() {
        let duration = Duration::from_secs(3661);
        let formatted = date::format_duration(duration);
        
        // Should be 01:01:01.00000 (1 hour, 1 minute, 1 second)
        assert!(formatted.contains("01:01:01"), "Duration should be formatted correctly");
    }

    #[test]
    fn test_format_duration_with_microseconds() {
        let duration = Duration::from_micros(3661000000);
        let formatted = date::format_duration(duration);
        
        assert!(formatted.contains("01:01:01"), "Duration with microseconds should be formatted correctly");
    }

    #[test]
    fn test_format_duration_zero() {
        let duration = Duration::from_secs(0);
        let formatted = date::format_duration(duration);
        
        assert_eq!(formatted, "00:00:00.00000");
    }
}

