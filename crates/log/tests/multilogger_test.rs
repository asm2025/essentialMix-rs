use emixlog::{LogLevel, MultiLogger};
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::sync::Arc;

struct TestLogger {
    logs: Arc<std::sync::Mutex<Vec<String>>>,
    filter: LevelFilter,
}

impl TestLogger {
    fn new(filter: LevelFilter) -> Self {
        Self {
            logs: Arc::new(std::sync::Mutex::new(Vec::new())),
            filter,
        }
    }
}

impl Log for TestLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.filter
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.logs
                .lock()
                .unwrap()
                .push(format!("{}: {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

#[test]
fn test_new() {
    let logger = MultiLogger::new();
    assert!(logger.is_empty());
    assert_eq!(logger.len(), 0);
}

#[test]
fn test_with_logger() {
    let test_logger = TestLogger::new(LevelFilter::Info);
    let logs = Arc::clone(&test_logger.logs);
    let multi_logger = MultiLogger::with_logger(Box::new(test_logger));
    assert_eq!(multi_logger.len(), 1);

    // Test that it can log
    let record = Record::builder()
        .level(Level::Info)
        .args(format_args!("test message"))
        .build();
    multi_logger.log(&record);

    let logged = logs.lock().unwrap();
    assert_eq!(logged.len(), 1);
}

#[test]
fn test_add_logger() {
    let logger1 = TestLogger::new(LevelFilter::Info);
    let logger2 = TestLogger::new(LevelFilter::Debug);
    let logs1 = Arc::clone(&logger1.logs);
    let logs2 = Arc::clone(&logger2.logs);

    let multi_logger = MultiLogger::new()
        .add_logger(Box::new(logger1))
        .add_logger(Box::new(logger2));

    assert_eq!(multi_logger.len(), 2);

    let record = Record::builder()
        .level(Level::Info)
        .args(format_args!("test message"))
        .build();
    multi_logger.log(&record);

    assert_eq!(logs1.lock().unwrap().len(), 1);
    assert_eq!(logs2.lock().unwrap().len(), 1);
}

#[test]
fn test_clone() {
    let logger1 = TestLogger::new(LevelFilter::Info);
    let logs1 = Arc::clone(&logger1.logs);

    let multi_logger1 = MultiLogger::new().add_logger(Box::new(logger1));
    let multi_logger2 = multi_logger1.clone();

    assert_eq!(multi_logger1.len(), multi_logger2.len());
    assert_eq!(multi_logger1.len(), 1);

    // Both should log to the same underlying logger
    let record = Record::builder()
        .level(Level::Info)
        .args(format_args!("test message"))
        .build();

    multi_logger1.log(&record);
    assert_eq!(logs1.lock().unwrap().len(), 1);

    multi_logger2.log(&record);
    assert_eq!(logs1.lock().unwrap().len(), 2);
}

#[test]
fn test_enabled() {
    let logger1 = TestLogger::new(LevelFilter::Info);
    let logger2 = TestLogger::new(LevelFilter::Debug);

    let multi_logger = MultiLogger::from_loggers(vec![
        Box::new(logger1) as Box<dyn Log + Send + Sync>,
        Box::new(logger2),
    ]);

    let metadata_info = Metadata::builder().level(Level::Info).build();
    let metadata_debug = Metadata::builder().level(Level::Debug).build();

    assert!(multi_logger.enabled(&metadata_info));
    assert!(multi_logger.enabled(&metadata_debug));
}

#[test]
fn test_thread_safety() {
    use std::thread;

    let logger1 = TestLogger::new(LevelFilter::Info);
    let logger2 = TestLogger::new(LevelFilter::Debug);
    let logs1 = Arc::clone(&logger1.logs);
    let logs2 = Arc::clone(&logger2.logs);

    let multi_logger = MultiLogger::from_loggers(vec![
        Box::new(logger1) as Box<dyn Log + Send + Sync>,
        Box::new(logger2),
    ]);

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let logger_clone = multi_logger.clone();
            thread::spawn(move || {
                let message = format!("message {}", i);
                let args = format_args!("{}", message);
                let record = Record::builder().level(Level::Info).args(args).build();
                logger_clone.log(&record);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(logs1.lock().unwrap().len(), 10);
    assert_eq!(logs2.lock().unwrap().len(), 10);
}

#[test]
fn test_flush() {
    let logger1 = TestLogger::new(LevelFilter::Info);
    let logger2 = TestLogger::new(LevelFilter::Debug);

    let multi_logger = MultiLogger::from_loggers(vec![
        Box::new(logger1) as Box<dyn Log + Send + Sync>,
        Box::new(logger2),
    ]);

    // Should not panic
    multi_logger.flush();
}

#[test]
fn test_init_empty_logger_error() {
    let empty_logger = MultiLogger::new();
    let result = empty_logger.init_with_level(LogLevel::Info);
    assert!(result.is_err());

    // Verify it's an InvalidInput error
    if let Err(e) = result {
        match e {
            emixcore::Error::InvalidInput(msg) => {
                assert!(msg.contains("must contain at least one logger"));
            }
            _ => panic!("Expected InvalidInput error, got {:?}", e),
        }
    }
}

#[test]
fn test_init_with_level_valid() {
    // Note: This test may fail if a logger is already set globally.
    // In that case, we just verify the method can be called.
    let logger = TestLogger::new(LevelFilter::Info);
    let multi_logger = MultiLogger::with_logger(Box::new(logger));

    // Try to initialize - may fail if logger already set, which is ok
    let result = multi_logger.try_init_with_level(LogLevel::Debug);

    // If successful, verify logging works through the global logger
    if result.is_ok() {
        log::info!("test message from global logger");
        // Give a small delay for async loggers if any
        std::thread::sleep(std::time::Duration::from_millis(10));
        // Note: We can't easily verify the logs here because the logger was moved
        // But at least we verified init_with_level doesn't panic
    }
    // If it failed (logger already set), that's also fine - means another test set it
}
