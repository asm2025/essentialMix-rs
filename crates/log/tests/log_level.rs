#[cfg(test)]
mod tests {
    use emixlog::LogLevel;
    use log::LevelFilter;

    #[test]
    fn test_log_level_default() {
        assert!(LogLevel::default() == LogLevel::Default);
    }

    #[test]
    fn test_log_level_from_to_level_filter_off() {
        let level: LevelFilter = LogLevel::Off.into();
        assert_eq!(level, LevelFilter::Off);
    }

    #[test]
    fn test_log_level_from_to_level_filter_trace() {
        let level: LevelFilter = LogLevel::Trace.into();
        assert_eq!(level, LevelFilter::Trace);
    }

    #[test]
    fn test_log_level_from_to_level_filter_debug() {
        let level: LevelFilter = LogLevel::Debug.into();
        assert_eq!(level, LevelFilter::Debug);
    }

    #[test]
    fn test_log_level_from_to_level_filter_info() {
        let level: LevelFilter = LogLevel::Info.into();
        assert_eq!(level, LevelFilter::Info);
    }

    #[test]
    fn test_log_level_from_to_level_filter_warn() {
        let level: LevelFilter = LogLevel::Warn.into();
        assert_eq!(level, LevelFilter::Warn);
    }

    #[test]
    fn test_log_level_from_to_level_filter_error() {
        let level: LevelFilter = LogLevel::Error.into();
        assert_eq!(level, LevelFilter::Error);
    }

    #[test]
    fn test_log_level_from_to_level_filter_critical() {
        let level: LevelFilter = LogLevel::Critical.into();
        assert_eq!(level, LevelFilter::Error); // Critical maps to Error
    }

    #[test]
    fn test_log_level_from_to_level_filter_default() {
        let level: LevelFilter = LogLevel::Default.into();
        assert_eq!(level, LevelFilter::Info); // Default maps to Info
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Off < LogLevel::Trace);
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Critical);
    }

    #[test]
    fn test_log_level_partial_eq() {
        assert!(LogLevel::Off == LogLevel::Off);
        assert!(LogLevel::Off != LogLevel::Trace);
    }

    #[test]
    fn test_log_level_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(LogLevel::Trace, "trace");
        map.insert(LogLevel::Debug, "debug");

        assert_eq!(map.get(&LogLevel::Trace), Some(&"trace"));
        assert_eq!(map.get(&LogLevel::Debug), Some(&"debug"));
        assert_eq!(map.get(&LogLevel::Info), None);
    }

    #[test]
    fn test_log_level_copy() {
        let level = LogLevel::Info;
        let copied = level;
        // Since LogLevel implements Copy, we can just verify they're equal
        assert!(level == copied);
        // Verify the copied value is Info by converting to LevelFilter
        let filter: LevelFilter = copied.into();
        assert_eq!(filter, LevelFilter::Info);
    }
}

