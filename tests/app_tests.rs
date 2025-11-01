#[cfg(test)]
mod tests {
    use emix::app::AppInfo;

    #[test]
    fn test_app_info() {
        let app_info = AppInfo::lib_info();
        // Assert that app_info is not empty
        assert!(!app_info.to_string().is_empty(), "App info should not be empty");
        // The exact format depends on AppInfo implementation, but we can verify it's a valid string
        assert!(app_info.to_string().len() > 0);
    }
}

