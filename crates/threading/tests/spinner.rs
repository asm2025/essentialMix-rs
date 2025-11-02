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

    #[test]
    fn test_spinner_finish_with_message() {
        let spinner = Spinner::new();
        
        assert!(!spinner.is_finished(), "Should not be finished initially");
        
        let result = spinner.finish_with_message("Done!");
        assert!(result.is_ok(), "finish_with_message should succeed");
        assert!(spinner.is_finished(), "Should be finished after finish_with_message()");
        
        // Should fail if already finished
        let result = spinner.finish_with_message("Done again!");
        assert!(result.is_err(), "finish_with_message should fail if already finished");
    }

    #[test]
    fn test_spinner_finish_using_style() {
        let spinner = Spinner::new();
        
        assert!(!spinner.is_finished(), "Should not be finished initially");
        
        let result = spinner.finish_using_style();
        assert!(result.is_ok(), "finish_using_style should succeed");
        assert!(spinner.is_finished(), "Should be finished after finish_using_style()");
        
        // Should fail if already finished
        let result = spinner.finish_using_style();
        assert!(result.is_err(), "finish_using_style should fail if already finished");
    }

    #[test]
    fn test_spinner_finish_and_clear() {
        let spinner = Spinner::new();
        spinner.set_message("Processing...");
        
        assert!(!spinner.is_finished(), "Should not be finished initially");
        
        let result = spinner.finish_and_clear();
        assert!(result.is_ok(), "finish_and_clear should succeed");
        assert!(spinner.is_finished(), "Should be finished after finish_and_clear()");
        
        // Should fail if already finished
        let result = spinner.finish_and_clear();
        assert!(result.is_err(), "finish_and_clear should fail if already finished");
    }

    #[test]
    fn test_spinner_abandon() {
        let spinner = Spinner::new();
        
        assert!(!spinner.is_finished(), "Should not be finished initially");
        
        let result = spinner.abandon();
        assert!(result.is_ok(), "abandon should succeed");
        assert!(spinner.is_finished(), "Should be finished after abandon()");
        
        // Should fail if already finished
        let result = spinner.abandon();
        assert!(result.is_err(), "abandon should fail if already finished");
    }

    #[test]
    fn test_spinner_abandon_with_message() {
        let spinner = Spinner::new();
        
        assert!(!spinner.is_finished(), "Should not be finished initially");
        
        let result = spinner.abandon_with_message("Abandoned".to_string());
        assert!(result.is_ok(), "abandon_with_message should succeed");
        assert!(spinner.is_finished(), "Should be finished after abandon_with_message()");
        
        // Should fail if already finished
        let result = spinner.abandon_with_message("Abandoned again".to_string());
        assert!(result.is_err(), "abandon_with_message should fail if already finished");
    }

    #[test]
    fn test_spinner_reset_elapsed() {
        let spinner = Spinner::new();
        spinner.reset_elapsed(); // Should not panic
    }

    #[test]
    fn test_spinner_reset_eta() {
        let spinner = Spinner::new();
        spinner.reset_eta(); // Should not panic
    }

    #[test]
    fn test_spinner_duration() {
        let spinner = Spinner::new();
        let _duration = spinner.duration(); // Should not panic
    }

    #[test]
    fn test_spinner_eta() {
        let spinner = Spinner::new();
        let _eta = spinner.eta(); // Should not panic
    }

    #[test]
    fn test_spinner_with_style() {
        use indicatif::ProgressStyle;
        
        let style = ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
        
        let spinner = Spinner::with_style(style);
        spinner.tick(); // Should not panic
    }

    #[test]
    fn test_spinner_with_finish() {
        use indicatif::ProgressFinish;
        
        let finish = ProgressFinish::WithMessage("Done".into());
        let spinner = Spinner::with_finish(finish);
        spinner.tick(); // Should not panic
    }

    #[test]
    fn test_spinner_with_options() {
        use emixthreading::SpinnerOptions;
        
        let mut options = SpinnerOptions::default();
        options.prefix = Some("TEST:".to_string());
        options.message = Some("Testing...".to_string());
        
        let spinner = Spinner::with_options(options);
        assert_eq!(spinner.prefix(), "TEST:");
        assert_eq!(spinner.message(), "Testing...");
    }

    #[test]
    fn test_spinner_with() {
        use indicatif::ProgressStyle;
        use emixthreading::SpinnerOptions;
        
        let style = ProgressStyle::with_template("{spinner} {msg}").unwrap();
        let mut options = SpinnerOptions::default();
        options.style = Some(style);
        
        let spinner = Spinner::with(None, None, options);
        spinner.tick(); // Should not panic
    }
}

