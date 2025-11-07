#[cfg(test)]
mod tests {
    use emix::app::AppInfo;

    #[test]
    fn test_app_info_lib_info() {
        let app_info = AppInfo::lib_info();
        // Assert that app_info is not empty
        assert!(
            !app_info.to_string().is_empty(),
            "App info should not be empty"
        );
        // The exact format depends on AppInfo implementation, but we can verify it's a valid string
        assert!(app_info.to_string().len() > 0);
    }

    #[test]
    fn test_app_info_new() {
        let app_info = AppInfo::new(
            "TestApp",
            "1.0.0",
            "Test Author",
            Some("Test Description"),
            Some("MIT"),
        );

        assert_eq!(app_info.name, "TestApp");
        assert_eq!(app_info.version, "1.0.0");
        assert_eq!(app_info.authors, "Test Author");
        assert_eq!(app_info.description, "Test Description");
        assert_eq!(app_info.license, "MIT");

        let display = app_info.to_string();
        assert!(display.contains("TestApp"));
        assert!(display.contains("1.0.0"));
        assert!(display.contains("Test Author"));
        assert!(display.contains("Test Description"));
        assert!(display.contains("License: MIT"));
    }

    #[test]
    fn test_app_info_new_with_optional_none() {
        let app_info = AppInfo::new("TestApp", "1.0.0", "Test Author", None, None);

        assert_eq!(app_info.description, "");
        assert_eq!(app_info.license, "");

        let display = app_info.to_string();
        assert!(display.contains("TestApp"));
        assert!(display.contains("1.0.0"));
        assert!(!display.contains("License:"));
    }

    #[test]
    fn test_app_info_clone_eq() {
        let app_info1 = AppInfo::new("Test", "1.0", "Author", None, None);
        let app_info2 = app_info1.clone();

        assert_eq!(app_info1.name, app_info2.name);
        assert_eq!(app_info1.version, app_info2.version);
    }

    #[test]
    fn test_app_info_default() {
        let app_info = AppInfo::default();
        assert_eq!(app_info.name, "");
        assert_eq!(app_info.version, "");
        assert_eq!(app_info.authors, "");
        assert_eq!(app_info.description, "");
        assert_eq!(app_info.license, "");
    }
}
