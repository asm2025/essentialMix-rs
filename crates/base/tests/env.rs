#[cfg(test)]
mod tests {
    use emix::env::*;
    use std::env;

    // Helper function to clean up environment variables after tests
    fn cleanup_env(key: &str) {
        unsafe {
            env::remove_var(key);
        }
    }

    #[test]
    fn test_get_env_when_set() {
        unsafe {
            env::set_var("TEST_VAR", "test_value");
        }
        let result = get_env("TEST_VAR");
        assert_eq!(result, Some("test_value".to_string()));
        cleanup_env("TEST_VAR");
    }

    #[test]
    fn test_get_env_when_not_set() {
        unsafe {
            env::remove_var("NONEXISTENT_VAR");
        }
        let result = get_env("NONEXISTENT_VAR");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_env_or_when_set() {
        unsafe {
            env::set_var("TEST_VAR", "actual_value");
        }
        let result = get_env_or("TEST_VAR", "default_value");
        assert_eq!(result, "actual_value");
        cleanup_env("TEST_VAR");
    }

    #[test]
    fn test_get_env_or_when_not_set() {
        unsafe {
            env::remove_var("NONEXISTENT_VAR");
        }
        let result = get_env_or("NONEXISTENT_VAR", "default_value");
        assert_eq!(result, "default_value");
    }

    #[test]
    fn test_get_required_env_when_set() {
        unsafe {
            env::set_var("REQUIRED_VAR", "required_value");
        }
        let result = get_required_env("REQUIRED_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "required_value");
        cleanup_env("REQUIRED_VAR");
    }

    #[test]
    fn test_get_required_env_when_not_set() {
        unsafe {
            env::remove_var("NONEXISTENT_REQUIRED_VAR");
        }
        let result = get_required_env("NONEXISTENT_REQUIRED_VAR");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Required environment variable NONEXISTENT_REQUIRED_VAR is not set")
        );
    }

    #[test]
    fn test_get_port_when_set() {
        unsafe {
            env::set_var("PORT", "8080");
        }
        let result = get_port();
        assert_eq!(result, 8080);
        cleanup_env("PORT");
    }

    #[test]
    fn test_get_port_when_not_set() {
        unsafe {
            env::remove_var("PORT");
        }
        let result = get_port();
        assert_eq!(result, 80); // Default value
    }

    #[test]
    fn test_get_port_with_invalid_value() {
        unsafe {
            env::set_var("PORT", "invalid");
        }
        let result = get_port();
        assert_eq!(result, 80); // Falls back to default on parse error
        cleanup_env("PORT");
    }

    #[test]
    fn test_get_port_or_when_set() {
        unsafe {
            env::set_var("PORT", "3000");
        }
        let result = get_port_or(8080);
        assert_eq!(result, 3000);
        cleanup_env("PORT");
    }

    #[test]
    fn test_get_port_or_when_not_set() {
        unsafe {
            env::remove_var("PORT");
        }
        let result = get_port_or(8080);
        assert_eq!(result, 8080); // Returns provided default
    }

    #[test]
    fn test_get_port_or_with_invalid_value() {
        unsafe {
            env::set_var("PORT", "not_a_number");
        }
        let result = get_port_or(8080);
        assert_eq!(result, 8080); // Returns default when parse fails
        cleanup_env("PORT");
    }

    #[test]
    fn test_get_database_url_when_set() {
        unsafe {
            env::set_var("DATABASE_URL", "postgresql://localhost/test");
        }
        let result = get_database_url();
        assert_eq!(result, Some("postgresql://localhost/test".to_string()));
        cleanup_env("DATABASE_URL");
    }

    #[test]
    fn test_get_database_url_when_not_set() {
        unsafe {
            env::remove_var("DATABASE_URL");
        }
        let result = get_database_url();
        assert_eq!(result, None);
    }

    #[test]
    fn test_is_development_when_set() {
        unsafe {
            env::set_var("NODE_ENV", "development");
        }
        let result = is_development();
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_development_when_not_set() {
        unsafe {
            env::remove_var("NODE_ENV");
        }
        let result = is_development();
        assert!(!result);
    }

    #[test]
    fn test_is_development_case_insensitive() {
        unsafe {
            env::set_var("NODE_ENV", "DEVELOPMENT");
        }
        let result = is_development();
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_staging_when_set() {
        unsafe {
            env::set_var("NODE_ENV", "staging");
        }
        let result = is_staging();
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_staging_when_not_set() {
        unsafe {
            env::remove_var("NODE_ENV");
        }
        let result = is_staging();
        assert!(!result);
    }

    #[test]
    fn test_is_staging_case_insensitive() {
        unsafe {
            env::set_var("NODE_ENV", "STAGING");
        }
        let result = is_staging();
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_production_when_set() {
        unsafe {
            env::set_var("NODE_ENV", "production");
        }
        let result = is_production();
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_production_when_not_set() {
        unsafe {
            env::remove_var("NODE_ENV");
        }
        let result = is_production();
        assert!(!result);
    }

    #[test]
    fn test_is_production_case_insensitive() {
        unsafe {
            env::set_var("NODE_ENV", "PRODUCTION");
        }
        let result = is_production();
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_environment_when_matches() {
        unsafe {
            env::set_var("NODE_ENV", "test");
        }
        let result = is_environment("test");
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_environment_when_not_matches() {
        unsafe {
            env::set_var("NODE_ENV", "production");
        }
        let result = is_environment("development");
        assert!(!result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_environment_when_not_set() {
        unsafe {
            env::remove_var("NODE_ENV");
        }
        let result = is_environment("test");
        assert!(!result);
    }

    #[test]
    fn test_is_environment_case_insensitive() {
        unsafe {
            env::set_var("NODE_ENV", "TEST");
        }
        let result = is_environment("test");
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_environment_empty_string() {
        unsafe {
            env::remove_var("NODE_ENV");
        }
        let result = is_environment("");
        assert!(!result);
    }

    #[test]
    fn test_is_any_environment_when_one_matches() {
        unsafe {
            env::set_var("NODE_ENV", "production");
        }
        let result = is_any_environment(&["development", "staging", "production"]);
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_any_environment_when_none_matches() {
        unsafe {
            env::set_var("NODE_ENV", "test");
        }
        let result = is_any_environment(&["development", "staging", "production"]);
        assert!(!result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_is_any_environment_when_not_set() {
        unsafe {
            env::remove_var("NODE_ENV");
        }
        let result = is_any_environment(&["development", "staging", "production"]);
        assert!(!result);
    }

    #[test]
    fn test_is_any_environment_case_insensitive() {
        unsafe {
            env::set_var("NODE_ENV", "PRODUCTION");
        }
        let result = is_any_environment(&["development", "staging", "production"]);
        assert!(result);
        cleanup_env("NODE_ENV");
    }

    #[test]
    fn test_get_allow_anonymous_users_when_true() {
        unsafe {
            env::set_var("ALLOW_ANONYMOUS_USERS", "true");
        }
        let result = get_allow_anonymous_users();
        assert!(result);
        cleanup_env("ALLOW_ANONYMOUS_USERS");
    }

    #[test]
    fn test_get_allow_anonymous_users_when_false() {
        unsafe {
            env::set_var("ALLOW_ANONYMOUS_USERS", "false");
        }
        let result = get_allow_anonymous_users();
        assert!(!result);
        cleanup_env("ALLOW_ANONYMOUS_USERS");
    }

    #[test]
    fn test_get_allow_anonymous_users_when_not_set() {
        unsafe {
            env::remove_var("ALLOW_ANONYMOUS_USERS");
        }
        let result = get_allow_anonymous_users();
        assert!(result); // Defaults to true when not set
    }

    #[test]
    fn test_get_allow_anonymous_users_when_other_value() {
        unsafe {
            env::set_var("ALLOW_ANONYMOUS_USERS", "yes");
        }
        let result = get_allow_anonymous_users();
        assert!(result); // Only "false" is treated as false, everything else is true
        cleanup_env("ALLOW_ANONYMOUS_USERS");
    }
}
