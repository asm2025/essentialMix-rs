#[cfg(feature = "slog")]
#[test]
fn test_slog_from_code() -> emixcore::Result<()> {
    use emix::io::path::IntoPath;
    use emixlog::slogx;
    use log::{debug, error, info, trace, warn};
    use std::fs;

    let log_path = ("_logs_test", "test_slog.log").into_path();

    // Clean up if exists
    if log_path.exists() {
        let _ = fs::remove_file(&log_path);
    }
    if let Some(parent) = log_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let _guard = slogx::build(&log_path)?;

    error!("Test error message");
    warn!("Test warning message");
    info!("Test info message");
    debug!("Test debug message");
    trace!("Test trace message");

    // Verify log file was created
    assert!(log_path.exists(), "Log file should exist after logging");

    // Clean up
    drop(_guard);
    if log_path.exists() {
        let _ = fs::remove_file(&log_path);
    }

    Ok(())
}

