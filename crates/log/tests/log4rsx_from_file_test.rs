use emixcore::{Error, Result};
use emixlog::log4rsx;
use log::{error, info, warn};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_log4rs_from_file() -> Result<()> {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/log4rs.yaml");

    if !config_path.exists() {
        return Err(Error::NotFound(config_path.to_string_lossy().into_owned()).into());
    }

    // Initialize logger from file - this will run in a separate process
    log4rsx::from_file(&config_path)?;

    error!("Test error from file config");
    warn!("Test warning from file config");
    info!("Test info from file config");

    // Clean up
    if config_path.exists() {
        let _ = fs::remove_file(
            config_path
                .parent()
                .unwrap()
                .join("_logs_test/test_file_config.log"),
        );
    }

    Ok(())
}
