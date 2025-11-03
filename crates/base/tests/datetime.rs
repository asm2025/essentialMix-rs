use chrono::{Datelike, Timelike, Utc};
use emix::datetime;
use std::time::Duration;

#[test]
fn test_parse_date() {
    let date_str = "2024-01-15 14:30:00";
    let result = datetime::parse_date(date_str);
    assert!(result.is_ok(), "Should parse date successfully");

    let dt = result.unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 1);
    assert_eq!(dt.day(), 15);
}

#[test]
fn test_parse_date_any_with_datetime_format() {
    let date_str = "2024-01-15 14:30";
    let result = datetime::parse_date_any(date_str);
    assert!(result.is_ok(), "Should parse date with datetime format");
}

#[test]
fn test_parse_date_any_with_long_format() {
    let date_str = "2024-01-15 14:30:00";
    let result = datetime::parse_date_any(date_str);
    assert!(result.is_ok(), "Should parse date with long format");
}

#[test]
fn test_parse_date_any_with_full_format() {
    let date_str = "2024-01-15 14:30:00.123456";
    let result = datetime::parse_date_any(date_str);
    assert!(result.is_ok(), "Should parse date with full format");
}

#[test]
fn test_parse_date_any_with_tz_format() {
    let date_str = "2024-01-15T14:30:00.123456Z";
    let result = datetime::parse_date_any(date_str);
    assert!(result.is_ok(), "Should parse date with timezone format");
}

#[test]
fn test_parse_date_any_with_date_only() {
    let date_str = "2024-01-15";
    let result = datetime::parse_date_any(date_str);
    assert!(result.is_ok(), "Should parse date only format");

    let dt = result.unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 1);
    assert_eq!(dt.day(), 15);
}

#[test]
fn test_parse_date_ftz() {
    let date_str = "2024-01-15T14:30:00.123456Z";
    let result = datetime::parse_date_ftz(date_str);
    assert!(result.is_ok(), "Should parse full timezone format");
}

#[test]
fn test_utc_today() {
    let today = datetime::utc_today();
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
    let formatted = datetime::format_duration(duration);

    // Should be in HH:MM:SS.ffffff format (timezone-dependent since it uses format_seconds_long)
    assert!(formatted.contains(":"), "Should contain colons");
    assert!(formatted.contains("."), "Should contain decimal point");
    // Should have 6 decimal places
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts.len(), 2, "Should have decimal part");
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
}

#[test]
fn test_format_duration_with_microseconds() {
    let duration = Duration::from_micros(3661000000);
    let formatted = datetime::format_duration(duration);

    // Should be in correct format
    assert!(formatted.contains(":"), "Should contain colons");
    assert!(formatted.contains("."), "Should contain decimal point");
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
}

#[test]
fn test_format_duration_zero() {
    let duration = Duration::from_secs(0);
    let formatted = datetime::format_duration(duration);

    // Should be in HH:MM:SS.ffffff format (timezone-dependent)
    assert!(formatted.contains(":"), "Should contain colons");
    assert!(formatted.contains("."), "Should contain decimal point");
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts.len(), 2, "Should have decimal part");
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
    // Microseconds should be all zeros
    assert_eq!(parts[1], "000000", "Microseconds should be zeros");
}

#[test]
fn test_format_duration_relative() {
    // Test that duration formatting preserves relative differences
    let base = Duration::from_secs(0);
    let one_hour = Duration::from_secs(3600);
    let one_hour_one_sec = Duration::from_secs(3601);

    let base_formatted = datetime::format_duration(base);
    let hour_formatted = datetime::format_duration(one_hour);
    let hour_sec_formatted = datetime::format_duration(one_hour_one_sec);

    // All should have correct format
    assert!(base_formatted.contains("."));
    assert!(hour_formatted.contains("."));
    assert!(hour_sec_formatted.contains("."));
    // They should be different (though modulo 24 hours)
    assert_ne!(base_formatted, hour_formatted);
    assert_ne!(hour_formatted, hour_sec_formatted);
}

#[test]
fn test_format_seconds() {
    // Test with a Unix timestamp that should produce a valid time format
    let timestamp = 1704067200; // 2024-01-01 00:00:00 UTC
    let formatted = datetime::format_seconds(timestamp);

    // Should be in HH:MM:SS format (timezone-dependent)
    assert_eq!(formatted.len(), 8, "Should be 8 characters (HH:MM:SS)");
    assert!(
        formatted.matches(':').count() == 2,
        "Should contain 2 colons"
    );
    assert!(!formatted.is_empty());
}

#[test]
fn test_format_seconds_zero() {
    // Test epoch timestamp (1970-01-01 00:00:00 UTC)
    let formatted = datetime::format_seconds(0);

    // Format should be valid HH:MM:SS (timezone-dependent)
    assert_eq!(formatted.len(), 8, "Should be 8 characters (HH:MM:SS)");
    assert!(
        formatted.matches(':').count() == 2,
        "Should contain 2 colons"
    );
}

#[test]
fn test_format_seconds_negative() {
    // Negative values should still format (though might show invalid time)
    let formatted = datetime::format_seconds(-1);
    // The function might return "invalid time" for negative values
    assert!(!formatted.is_empty());
}

#[test]
fn test_format_seconds_relative() {
    // Test that relative time differences work correctly
    let base = 1704067200; // 2024-01-01 00:00:00 UTC
    let one_hour_later = base + 3600;

    let base_formatted = datetime::format_seconds(base);
    let later_formatted = datetime::format_seconds(one_hour_later);

    // Both should be valid time formats
    assert_eq!(base_formatted.len(), 8);
    assert_eq!(later_formatted.len(), 8);
    // The later one should be 1 hour ahead (modulo 24 hours)
    assert_ne!(base_formatted, later_formatted);
}

#[test]
fn test_format_seconds_format() {
    // Test that the format is correct HH:MM:SS
    let timestamp = 1704067200;
    let formatted = datetime::format_seconds(timestamp);

    // Check format: should match HH:MM:SS pattern
    let parts: Vec<&str> = formatted.split(':').collect();
    assert_eq!(parts.len(), 3, "Should have 3 parts separated by colons");
    assert_eq!(parts[0].len(), 2, "Hours should be 2 digits");
    assert_eq!(parts[1].len(), 2, "Minutes should be 2 digits");
    assert_eq!(parts[2].len(), 2, "Seconds should be 2 digits");
}

#[test]
fn test_format_seconds_long() {
    // Test with a Unix timestamp in microseconds (2024-01-01 00:00:00 UTC)
    let timestamp = 1704067200_000000u128;
    let formatted = datetime::format_seconds_long(timestamp);

    // Should contain time format and decimal point
    assert!(formatted.contains(":"), "Should contain colons");
    assert!(formatted.contains("."), "Should contain decimal point");
    // Should have microseconds (6 digits after decimal)
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts.len(), 2, "Should have decimal part");
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
}

#[test]
fn test_format_seconds_long_zero() {
    // Test epoch timestamp (1970-01-01 00:00:00 UTC)
    let formatted = datetime::format_seconds_long(0);

    // Format should be HH:MM:SS.ffffff (timezone-dependent)
    assert!(formatted.contains(":"), "Should contain colons");
    assert!(formatted.contains("."), "Should contain decimal point");
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts.len(), 2, "Should have decimal part");
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
}

#[test]
fn test_format_seconds_long_with_microseconds() {
    // Test with microseconds: 2024-01-01 00:00:00.123456 UTC
    let timestamp = 1704067200_123456u128;
    let formatted = datetime::format_seconds_long(timestamp);

    // Should have the microseconds part
    assert!(formatted.contains("."), "Should contain decimal point");
    assert!(formatted.contains("123456"), "Should contain microseconds");
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts[1], "123456", "Microseconds should be preserved");
}

#[test]
fn test_format_seconds_long_partial_microseconds() {
    // Test that microseconds are properly formatted even when less than 6 digits
    // This tests the conversion from microseconds to nanoseconds
    let timestamp = 1704067200_000123u128; // 123 microseconds
    let formatted = datetime::format_seconds_long(timestamp);

    // Should contain the microseconds (will be padded with zeros)
    assert!(formatted.contains("."), "Should contain decimal point");
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
    // The microseconds part should end with 123
    assert!(parts[1].ends_with("123"), "Should contain microseconds");
}

#[test]
fn test_format_seconds_long_relative() {
    // Test that relative time differences work correctly
    let base = 1704067200_000000u128; // 2024-01-01 00:00:00 UTC
    let one_hour_later = 1704070800_000000u128; // 1 hour later

    let base_formatted = datetime::format_seconds_long(base);
    let later_formatted = datetime::format_seconds_long(one_hour_later);

    // Both should be valid time formats
    assert!(base_formatted.contains(":"));
    assert!(later_formatted.contains(":"));
    assert!(base_formatted.contains("."));
    assert!(later_formatted.contains("."));
    // The later one should be 1 hour ahead (modulo 24 hours)
    assert_ne!(base_formatted, later_formatted);
}

#[test]
fn test_format_seconds_long_format() {
    // Test that the format is correct HH:MM:SS.ffffff
    let timestamp = 1704067200_123456u128;
    let formatted = datetime::format_seconds_long(timestamp);

    // Check format: should match HH:MM:SS.ffffff pattern
    let time_micros: Vec<&str> = formatted.split('.').collect();
    assert_eq!(
        time_micros.len(),
        2,
        "Should have time and microsecond parts"
    );

    let time_parts: Vec<&str> = time_micros[0].split(':').collect();
    assert_eq!(time_parts.len(), 3, "Should have 3 time parts");
    assert_eq!(time_parts[0].len(), 2, "Hours should be 2 digits");
    assert_eq!(time_parts[1].len(), 2, "Minutes should be 2 digits");
    assert_eq!(time_parts[2].len(), 2, "Seconds should be 2 digits");
    assert_eq!(time_micros[1].len(), 6, "Microseconds should be 6 digits");
}

// Tests for datetime::unix module
#[test]
fn test_unix_now() {
    let timestamp = datetime::unix::now();
    // Should be a reasonable unix timestamp (after 2000-01-01)
    assert!(timestamp > 946_684_800, "Should be after year 2000");
}

#[test]
fn test_unix_now_micros() {
    let timestamp = datetime::unix::now_micros();
    // Should be a reasonable unix timestamp in microseconds
    assert!(
        timestamp > 946_684_800_000_000,
        "Should be after year 2000 in microseconds"
    );
}

#[test]
fn test_unix_now_millis() {
    let timestamp = datetime::unix::now_millis();
    // Should be a reasonable unix timestamp in milliseconds
    assert!(
        timestamp > 946_684_800_000,
        "Should be after year 2000 in milliseconds"
    );
}

#[test]
fn test_unix_now_consistency() {
    // Test that the different now functions are consistent
    let now_secs = datetime::unix::now();
    let now_millis = datetime::unix::now_millis();
    let now_micros = datetime::unix::now_micros();

    // Allow for small differences due to timing
    let millis_from_secs = now_secs as u128 * 1000;
    let micros_from_secs = now_secs as u128 * 1_000_000;

    assert!(
        now_millis >= millis_from_secs && now_millis < millis_from_secs + 1000,
        "now_millis should be close to now() * 1000"
    );
    assert!(
        now_micros >= micros_from_secs && now_micros < micros_from_secs + 1_000_000,
        "now_micros should be close to now() * 1_000_000"
    );
}

#[test]
fn test_unix_to_system_time() {
    // Test with a known timestamp: 2024-01-01 00:00:00 UTC = 1704067200
    let timestamp = 1704067200;
    let system_time = datetime::unix::to_system_time(timestamp);

    // Convert back to verify
    let duration = system_time
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Should be valid duration");
    assert_eq!(duration.as_secs(), timestamp);
}

#[test]
fn test_unix_to_system_time_zero() {
    // Test with epoch timestamp
    let system_time = datetime::unix::to_system_time(0);
    let duration = system_time
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Should be valid duration");
    assert_eq!(duration.as_secs(), 0);
}

#[test]
fn test_format_duration_uses_format_seconds_long() {
    // Verify that format_duration calls format_seconds_long correctly
    let duration = Duration::from_micros(3661123456);
    let formatted = datetime::format_duration(duration);

    // Should match format_seconds_long output
    let expected = datetime::format_seconds_long(3661123456);
    assert_eq!(formatted, expected);
}

#[test]
fn test_format_duration_with_nanoseconds() {
    // Test that nanoseconds are properly converted to microseconds
    // Note: as_micros() truncates nanoseconds, so sub-microsecond precision is lost
    let duration = Duration::from_nanos(3661000000123);
    let formatted = datetime::format_duration(duration);

    // Should format correctly with proper structure
    assert!(formatted.contains(":"), "Should contain colons");
    assert!(formatted.contains("."), "Should contain decimal point");
    let parts: Vec<&str> = formatted.split('.').collect();
    assert_eq!(parts.len(), 2, "Should have decimal part");
    assert_eq!(parts[1].len(), 6, "Should have 6 decimal digits");
    // The function converts Duration to microseconds via as_micros() which truncates
    // nanoseconds, so we just verify the format is correct
}
