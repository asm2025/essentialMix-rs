use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::sync::Arc;

use crate::{LogLevel, Result};

/// A thread-safe, cloneable logger that forwards log records to multiple underlying loggers.
///
/// `MultiLogger` implements the `Log` trait and allows you to combine multiple loggers
/// into a single logger interface. All loggers in the collection will receive log records
/// that match their enabled state.
///
/// # Thread Safety
///
/// `MultiLogger` is `Send + Sync`, meaning it can be safely shared across thread boundaries.
/// The underlying loggers must also implement `Send + Sync`.
///
/// # Cloning
///
/// `MultiLogger` implements `Clone`, which creates a new handle to the same underlying loggers.
/// The loggers themselves are shared via `Arc`, so cloning is inexpensive.
///
/// # Example
///
/// ```no_run
/// use emixlog::MultiLogger;
/// use log::{Log, Metadata, Record};
///
/// struct MyLogger;
///
/// impl Log for MyLogger {
///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
///     fn log(&self, _record: &Record) {}
///     fn flush(&self) {}
/// }
///
/// let multi_logger = MultiLogger::new()
///     .add_logger(Box::new(MyLogger));
/// ```
#[derive(Clone)]
pub struct MultiLogger {
    loggers: Arc<Vec<Arc<dyn Log + Send + Sync>>>,
}

impl MultiLogger {
    /// Creates a new empty `MultiLogger`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// let logger = MultiLogger::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            loggers: Arc::new(Vec::new()),
        }
    }

    /// Creates a new `MultiLogger` with a single logger.
    ///
    /// # Arguments
    ///
    /// * `logger` - The initial logger to add to the collection
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let logger = MultiLogger::with_logger(Box::new(MyLogger));
    /// ```
    #[must_use]
    pub fn with_logger(logger: Box<dyn Log + Send + Sync>) -> Self {
        Self {
            loggers: Arc::new(vec![Arc::from(logger)]),
        }
    }

    /// Creates a new `MultiLogger` from a collection of loggers.
    ///
    /// # Arguments
    ///
    /// * `loggers` - A vector of loggers to initialize the collection with
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let loggers: Vec<Box<dyn Log + Send + Sync>> = vec![
    ///     Box::new(MyLogger),
    /// ];
    /// let logger = MultiLogger::from_loggers(loggers);
    /// ```
    #[must_use]
    pub fn from_loggers(loggers: Vec<Box<dyn Log + Send + Sync>>) -> Self {
        Self {
            loggers: Arc::new(
                loggers
                    .into_iter()
                    .map(|logger| Arc::from(logger))
                    .collect(),
            ),
        }
    }

    /// Adds a logger to the collection.
    ///
    /// This method creates a new `MultiLogger` with the additional logger.
    /// The original loggers are preserved and shared via `Arc`.
    ///
    /// # Arguments
    ///
    /// * `logger` - The logger to add to the collection
    ///
    /// # Returns
    ///
    /// A new `MultiLogger` instance containing all existing loggers plus the new one.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let logger = MultiLogger::new()
    ///     .add_logger(Box::new(MyLogger));
    /// ```
    #[must_use]
    pub fn add_logger(self, logger: Box<dyn Log + Send + Sync>) -> Self {
        // Get a mutable copy of the current loggers
        let mut new_loggers: Vec<Arc<dyn Log + Send + Sync>> =
            Arc::try_unwrap(self.loggers).unwrap_or_else(|arc| (*arc).clone());
        new_loggers.push(Arc::from(logger));
        Self {
            loggers: Arc::new(new_loggers),
        }
    }

    /// Returns the number of loggers in the collection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// let logger = MultiLogger::new();
    /// assert_eq!(logger.len(), 0);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.loggers.len()
    }

    /// Returns `true` if there are no loggers in the collection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// let logger = MultiLogger::new();
    /// assert!(logger.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.loggers.is_empty()
    }

    /// Initializes this `MultiLogger` as the global logger with the default log level (Info).
    ///
    /// This function will set the logger as the global logger using `log::set_boxed_logger`.
    /// The logger is kept alive for the lifetime of the program.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the logger was successfully set
    /// - `Err(SetLoggerError)` if a logger has already been set
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let multi_logger = MultiLogger::new()
    ///     .add_logger(Box::new(MyLogger));
    /// multi_logger.init().unwrap();
    /// ```
    pub fn init(self) -> Result<()> {
        self.init_with_level(LogLevel::Info)
    }

    /// Initializes this `MultiLogger` as the global logger with the specified log level.
    ///
    /// This function will set the logger as the global logger using `log::set_boxed_logger`
    /// and set the maximum log level filter using `log::set_max_level`.
    /// The logger is kept alive for the lifetime of the program.
    ///
    /// # Arguments
    ///
    /// * `level` - The maximum log level to filter
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the logger was successfully set
    /// - `Err(Error)` if a logger has already been set or if there was an error setting the logger
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::{MultiLogger, LogLevel};
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let multi_logger = MultiLogger::new()
    ///     .add_logger(Box::new(MyLogger));
    /// multi_logger.init_with_level(LogLevel::Debug).unwrap();
    /// ```
    pub fn init_with_level(self, level: LogLevel) -> Result<()> {
        if self.loggers.is_empty() {
            return Err(crate::Error::InvalidInput(
                "MultiLogger must contain at least one logger".to_string(),
            ));
        }

        let max_level: LevelFilter = level.into();
        log::set_max_level(max_level);

        // Convert MultiLogger to Box<dyn Log> for the global logger
        log::set_boxed_logger(Box::new(self)).map_err(|e| crate::Error::from_std_error(e))?;

        Ok(())
    }

    /// Attempts to initialize this `MultiLogger` as the global logger with the default log level (Info).
    ///
    /// This is a convenience wrapper around `init()` that returns `SetLoggerError` directly.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the logger was successfully set
    /// - `Err(SetLoggerError)` if a logger has already been set
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::MultiLogger;
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let multi_logger = MultiLogger::new()
    ///     .add_logger(Box::new(MyLogger));
    /// multi_logger.try_init().unwrap();
    /// ```
    pub fn try_init(self) -> std::result::Result<(), SetLoggerError> {
        self.try_init_with_level(LogLevel::Info)
    }

    /// Attempts to initialize this `MultiLogger` as the global logger with the specified log level.
    ///
    /// This is a convenience wrapper around `init_with_level()` that returns `SetLoggerError` directly.
    /// Note: If the MultiLogger is empty, this will still attempt to set it, but it won't log anything.
    ///
    /// # Arguments
    ///
    /// * `level` - The maximum log level to filter
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the logger was successfully set
    /// - `Err(SetLoggerError)` if a logger has already been set
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emixlog::{MultiLogger, LogLevel};
    /// use log::{Log, Metadata, Record};
    ///
    /// struct MyLogger;
    /// impl Log for MyLogger {
    ///     fn enabled(&self, _metadata: &Metadata) -> bool { true }
    ///     fn log(&self, _record: &Record) {}
    ///     fn flush(&self) {}
    /// }
    ///
    /// let multi_logger = MultiLogger::new()
    ///     .add_logger(Box::new(MyLogger));
    /// multi_logger.try_init_with_level(LogLevel::Debug).unwrap();
    /// ```
    pub fn try_init_with_level(self, level: LogLevel) -> std::result::Result<(), SetLoggerError> {
        let max_level: LevelFilter = level.into();
        log::set_max_level(max_level);

        // Convert MultiLogger to Box<dyn Log> for the global logger
        log::set_boxed_logger(Box::new(self))?;

        Ok(())
    }
}

impl Default for MultiLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.loggers.iter().any(|l| l.enabled(metadata))
    }

    fn log(&self, record: &Record) {
        for logger in self.loggers.iter() {
            if logger.enabled(record.metadata()) {
                logger.log(record);
            }
        }
    }

    fn flush(&self) {
        for logger in self.loggers.iter() {
            logger.flush();
        }
    }
}
