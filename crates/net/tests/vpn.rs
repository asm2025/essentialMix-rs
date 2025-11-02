#[cfg(test)]
mod tests {
    use emixcore::Result;

    #[cfg(feature = "vpn")]
    use emixnet::vpn::*;

    #[cfg(feature = "vpn")]
    use std::sync::atomic::{AtomicBool, Ordering};
    #[cfg(feature = "vpn")]
    use std::env;

    // Environment variable to enable/disable ExpressVPN tests
    // Set EXPRESSVPN_TESTS=1 to enable actual ExpressVPN tests
    #[cfg(feature = "vpn")]
    static EXPRESSVPN_TESTS_ENABLED: AtomicBool = AtomicBool::new(false);
    
    #[cfg(feature = "vpn")]
    fn are_expressvpn_tests_enabled() -> bool {
        // Check environment variable
        env::var("EXPRESSVPN_TESTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false)
    }

    #[cfg(feature = "vpn")]
    fn init_expressvpn_tests() {
        if !EXPRESSVPN_TESTS_ENABLED.load(Ordering::Relaxed) {
            EXPRESSVPN_TESTS_ENABLED.store(true, Ordering::Relaxed);
        }
    }

    #[test]
    #[cfg(feature = "vpn")]
    fn test_expressvpn_status_enum() {
        // Test ExpressVPNStatus enum variants
        let unknown = ExpressVPNStatus::Unknown;
        assert_eq!(unknown, ExpressVPNStatus::Unknown);

        let not_activated = ExpressVPNStatus::NotActivated;
        assert_eq!(not_activated, ExpressVPNStatus::NotActivated);

        let disconnected = ExpressVPNStatus::Disconnected;
        assert_eq!(disconnected, ExpressVPNStatus::Disconnected);

        let connected_none = ExpressVPNStatus::Connected(None);
        assert_eq!(connected_none, ExpressVPNStatus::Connected(None));

        let connected_some = ExpressVPNStatus::Connected(Some("Test Location".to_string()));
        match connected_some {
            ExpressVPNStatus::Connected(Some(location)) => assert_eq!(location, "Test Location"),
            _ => panic!("Should be Connected(Some)"),
        }

        let error = ExpressVPNStatus::Error("Test error".to_string());
        match error {
            ExpressVPNStatus::Error(msg) => assert_eq!(msg, "Test error"),
            _ => panic!("Should be Error"),
        }
    }

    #[test]
    #[cfg(feature = "vpn")]
    fn test_expressvpn_impl() {
        // Test that ExpressVPN can be instantiated
        let vpn = ExpressVPN;
        // Just verify the struct exists and can be created
        std::mem::drop(vpn);
    }

    #[cfg(feature = "vpn")]
    fn check_expressvpn_installed() -> bool {
        use std::process::Command;
        Command::new("expressvpn")
            .arg("--version")
            .output()
            .is_ok()
    }

    #[tokio::test]
    #[cfg(feature = "vpn")]
    async fn test_expressvpn_version_if_enabled() -> Result<()> {
        // Only run if ExpressVPN tests are enabled via environment variable
        if !are_expressvpn_tests_enabled() {
            println!("Skipping: EXPRESSVPN_TESTS environment variable not set");
            return Ok(());
        }

        init_expressvpn_tests();

        if !check_expressvpn_installed() {
            println!("Skipping: ExpressVPN not installed");
            return Ok(());
        }

        let vpn = ExpressVPN;
        match vpn.version() {
            Ok(version) => {
                println!("ExpressVPN version: {}", version);
                assert!(!version.is_empty());
                Ok(())
            }
            Err(e) => {
                println!("Failed to get ExpressVPN version: {:?}", e);
                Err(e)
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "vpn")]
    async fn test_expressvpn_status_if_enabled() -> Result<()> {
        // Only run if ExpressVPN tests are enabled via environment variable
        if !are_expressvpn_tests_enabled() {
            println!("Skipping: EXPRESSVPN_TESTS environment variable not set");
            return Ok(());
        }

        init_expressvpn_tests();

        if !check_expressvpn_installed() {
            println!("Skipping: ExpressVPN not installed");
            return Ok(());
        }

        let vpn = ExpressVPN;
        match vpn.status() {
            Ok(status) => {
                println!("ExpressVPN status: {:?}", status);
                Ok(())
            }
            Err(e) => {
                println!("Failed to get ExpressVPN status: {:?}", e);
                Err(e)
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "vpn")]
    async fn test_expressvpn_list_locations_if_enabled() -> Result<()> {
        // Only run if ExpressVPN tests are enabled via environment variable
        if !are_expressvpn_tests_enabled() {
            println!("Skipping: EXPRESSVPN_TESTS environment variable not set");
            return Ok(());
        }

        init_expressvpn_tests();

        if !check_expressvpn_installed() {
            println!("Skipping: ExpressVPN not installed");
            return Ok(());
        }

        let vpn = ExpressVPN;
        
        // Test list() function
        match vpn.list() {
            Ok(locations) => {
                println!("Found {} locations", locations.len());
                Ok(())
            }
            Err(e) => {
                println!("Failed to list locations: {:?}", e);
                Err(e)
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "vpn")]
    async fn test_expressvpn_recommended_locations_if_enabled() -> Result<()> {
        // Only run if ExpressVPN tests are enabled via environment variable
        if !are_expressvpn_tests_enabled() {
            println!("Skipping: EXPRESSVPN_TESTS environment variable not set");
            return Ok(());
        }

        init_expressvpn_tests();

        if !check_expressvpn_installed() {
            println!("Skipping: ExpressVPN not installed");
            return Ok(());
        }

        let vpn = ExpressVPN;
        
        // Test recommended() function
        match vpn.recommended() {
            Ok(locations) => {
                println!("Found {} recommended locations", locations.len());
                Ok(())
            }
            Err(e) => {
                println!("Failed to list recommended locations: {:?}", e);
                Err(e)
            }
        }
    }

    #[tokio::test]
    #[cfg(feature = "vpn")]
    async fn test_expressvpn_recent_locations_if_enabled() -> Result<()> {
        // Only run if ExpressVPN tests are enabled via environment variable
        if !are_expressvpn_tests_enabled() {
            println!("Skipping: EXPRESSVPN_TESTS environment variable not set");
            return Ok(());
        }

        init_expressvpn_tests();

        if !check_expressvpn_installed() {
            println!("Skipping: ExpressVPN not installed");
            return Ok(());
        }

        let vpn = ExpressVPN;
        
        // Test recent() function
        match vpn.recent() {
            Ok(locations) => {
                println!("Found {} recent locations", locations.len());
                Ok(())
            }
            Err(e) => {
                println!("Failed to list recent locations: {:?}", e);
                Err(e)
            }
        }
    }

    #[test]
    #[cfg(not(feature = "vpn"))]
    fn test_vpn_feature_disabled() {
        // Test that VPN tests are properly skipped when feature is disabled
        println!("VPN feature is disabled - tests skipped");
    }
}
