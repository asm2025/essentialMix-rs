mod error;
#[cfg(feature = "log4rs")]
pub mod log4rsx;
mod multilogger;
#[cfg(feature = "slog")]
pub mod slogx;
pub use multilogger::*;

pub use emixcore::{Error, Result};
pub use error::{LogError, Result as LogResult};

use log::LevelFilter;

pub const LOG_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S.%f";
pub const LOG_SIZE_MIN: usize = 1024 * 1024 * 2;
pub const LOG_SIZE_MAX: usize = 1024 * 1024 * 10;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Off,
    #[default]
    Default,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> LevelFilter {
        match level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Trace => LevelFilter::Trace,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Critical => LevelFilter::Error,
            _ => LevelFilter::Info,
        }
    }
}
