use thiserror::Error;
use emixcore::Error as CoreError;

/// Logging-specific error types
#[derive(Error, Debug)]
pub enum LogError {
    #[error("Log4rs configuration error: {0}")]
    Log4rsConfig(String),

    #[error("Log4rs initialization error: {0}")]
    Log4rsInit(String),

    #[error("Slog configuration error: {0}")]
    SlogConfig(String),

    #[error("Logger initialization error: {0}")]
    LoggerInit(String),

    #[error("File rotation error: {0}")]
    FileRotation(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

impl From<LogError> for CoreError {
    fn from(err: LogError) -> Self {
        match err {
            LogError::Log4rsConfig(msg) => CoreError::Other(format!("Log4rs configuration error: {}", msg)),
            LogError::Log4rsInit(msg) => CoreError::Other(format!("Log4rs initialization error: {}", msg)),
            LogError::SlogConfig(msg) => CoreError::Other(format!("Slog configuration error: {}", msg)),
            LogError::LoggerInit(msg) => CoreError::Other(format!("Logger initialization error: {}", msg)),
            LogError::FileRotation(msg) => CoreError::Other(format!("File rotation error: {}", msg)),
            LogError::OperationFailed(msg) => CoreError::Other(format!("Operation failed: {}", msg)),
        }
    }
}

/// Result type alias for logging operations
pub type Result<T> = std::result::Result<T, LogError>;

/// Helper functions for creating logging-specific errors
impl LogError {
    pub fn log4rs_config(msg: impl Into<String>) -> Self {
        LogError::Log4rsConfig(msg.into())
    }

    pub fn log4rs_init(msg: impl Into<String>) -> Self {
        LogError::Log4rsInit(msg.into())
    }

    pub fn slog_config(msg: impl Into<String>) -> Self {
        LogError::SlogConfig(msg.into())
    }

    pub fn logger_init(msg: impl Into<String>) -> Self {
        LogError::LoggerInit(msg.into())
    }

    pub fn file_rotation(msg: impl Into<String>) -> Self {
        LogError::FileRotation(msg.into())
    }
}

