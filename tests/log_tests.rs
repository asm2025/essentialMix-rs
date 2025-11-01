#[cfg(test)]
mod tests {
    use emix::{
        io::{directory, path::IntoPath},
        Result,
    };
    use emixlog::{log4rs, slog};
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
        
        log4rs::build(&log_path)?;
        
        error!("Test error message");
        warn!("Test warning message");
        info!("Test info message");
        debug!("Test debug message");
        trace!("Test trace message");
        
        // Verify log file was created
        assert!(log_path.exists(), "Log file should exist after logging");
        
        // Clean up
        if log_path.exists() {
            let _ = fs::remove_file(&log_path);
        }
        
        Ok(())
    }

    #[test]
    fn test_slog_from_code() -> Result<()> {
        let log_path = ("_logs_test", "test_slog.log").into_path();
        
        // Clean up if exists
        if log_path.exists() {
            let _ = fs::remove_file(&log_path);
        }
        if let Some(parent) = log_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        let _guard = slog::build(&log_path)?;
        
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

    #[test]
    #[ignore] // Manual test - requires log4rs.yaml config file
    fn test_log4rs_from_file() -> Result<()> {
        let config_path = directory::current()?.join("files/log/log4rs.yaml");
        
        if !config_path.exists() {
            return Ok(()); // Skip if config doesn't exist
        }
        
        log4rs::from_file(&config_path)?;
        
        error!("Test error from file config");
        warn!("Test warning from file config");
        info!("Test info from file config");
        
        Ok(())
    }
}

