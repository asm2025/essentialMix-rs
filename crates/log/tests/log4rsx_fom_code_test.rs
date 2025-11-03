use emix::io::path::IntoPath;
use emixcore::Result;
use emixlog::log4rsx;
use log::{debug, error, info, trace, warn};
use std::fs;

#[test]
fn test_log4rs_from_code() -> Result<()> {
    let log_path = ("_logs_test", "test_code.log").into_path();

    // Clean up if exists
    if log_path.exists() {
        let _ = fs::remove_file(&log_path);
    }
    if let Some(parent) = log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _handle = log4rsx::build(&log_path)?;

    error!("Test error message");
    warn!("Test warning message");
    info!("Test info message");
    debug!("Test debug message");
    trace!("Test trace message");

    // Wait a bit to ensure logs are flushed
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Verify log file was created and has content
    assert!(log_path.exists(), "Log file should exist after logging");
    let content = fs::read_to_string(&log_path).unwrap_or_default();
    assert!(!content.is_empty(), "Log file should not be empty");
    assert!(
        content.contains("Test error message"),
        "Log file should contain error message"
    );
    assert!(
        content.contains("Test info message"),
        "Log file should contain info message"
    );

    // Clean up
    drop(_handle);
    if log_path.exists() {
        let _ = fs::remove_file(&log_path);
    }

    Ok(())
}
