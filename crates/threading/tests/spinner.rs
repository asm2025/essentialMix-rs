#[cfg(test)]
mod tests {
    use emixthreading::Spinner;
    use std::time::Duration;

    #[test]
    fn test_spinner_new() {
        let spinner = Spinner::new();
        assert!(spinner.message().contains("Initializing"), "Should have default message");
    }

    #[test]
    fn test_spinner_with_prefix() {
        let spinner = Spinner::with_prefix("TEST:".to_string());
        assert_eq!(spinner.prefix(), "TEST:", "Should have correct prefix");
    }

    #[test]
    fn test_spinner_message_and_prefix() {
        let spinner = Spinner::new();
        
        spinner.set_message("Processing...");
        assert_eq!(spinner.message(), "Processing...");
        
        spinner.set_prefix("INFO:");
        assert_eq!(spinner.prefix(), "INFO:");
        
        spinner.clear_message();
        assert_eq!(spinner.message(), "");
        
        spinner.clear_prefix();
        assert_eq!(spinner.prefix(), "");
    }

    #[test]
    fn test_spinner_steady_tick() {
        let spinner = Spinner::new();
        
        spinner.set_steady_tick(50);
        // Should not panic
        
        spinner.set_steady_tick(0);
        // Should disable steady tick
    }

    #[test]
    fn test_spinner_tick() {
        let spinner = Spinner::new();
        spinner.tick(); // Should not panic
    }

    #[test]
    fn test_spinner_run() {
        let spinner = Spinner::new();
        
        let result = spinner.run(|| {
            std::thread::sleep(Duration::from_millis(100));
            42
        });
        
        assert!(result.is_ok(), "run should succeed");
        assert_eq!(result.unwrap(), 42, "should return correct value");
    }

    #[test]
    fn test_spinner_suspend() {
        let spinner = Spinner::new();
        
        let result = spinner.suspend(|| {
            "suspended"
        });
        
        assert_eq!(result, "suspended", "suspend should return correct value");
    }

    #[test]
    fn test_spinner_finish() {
        let spinner = Spinner::new();
        
        // Initially not finished
        assert!(!spinner.is_finished(), "Should not be finished initially");
        
        let result = spinner.finish();
        assert!(result.is_ok(), "finish should succeed");
        assert!(spinner.is_finished(), "Should be finished after finish()");
        
        // Finish() should fail if already finished
        let result = spinner.finish();
        assert!(result.is_err(), "finish should fail if already finished");
    }

    #[test]
    fn test_spinner_reset() {
        let spinner = Spinner::new();
        
        spinner.set_message("Test");
        let _ = spinner.finish();
        
        let result = spinner.reset();
        assert!(result.is_ok(), "reset should succeed");
        assert!(!spinner.is_finished(), "Should not be finished after reset");
    }

    #[test]
    fn test_spinner_elapsed() {
        let elapsed = Duration::from_secs(5);
        let spinner = Spinner::with_elapsed(elapsed);
        
        // Just verify it doesn't crash
        spinner.tick();
    }
}

