#[cfg(test)]
mod tests {
    use emixcore::Error;

    #[test]
    fn test_error_not_supported() {
        let err = Error::NotSupported;
        assert_eq!(err.to_string(), "Operation is not supported");
    }

    #[test]
    fn test_error_not_implemented() {
        let err = Error::NotImplemented;
        assert_eq!(err.to_string(), "Not implemented");
    }

    #[test]
    fn test_error_canceled() {
        let err = Error::Canceled;
        assert_eq!(err.to_string(), "Operation is canceled");
    }

    #[test]
    fn test_error_timeout() {
        let err = Error::Timeout;
        assert_eq!(err.to_string(), "Operation timed out");
    }

    #[test]
    fn test_error_not_found() {
        let err = Error::NotFound("test item".to_string());
        assert_eq!(err.to_string(), "Item not found. test item");
    }

    #[test]
    fn test_error_no_input() {
        let err = Error::NoInput;
        assert_eq!(err.to_string(), "No input was provided.");
    }

    #[test]
    fn test_error_invalid_input() {
        let err = Error::InvalidInput("test input".to_string());
        assert_eq!(err.to_string(), "Invalid input. test input");
    }

    #[test]
    fn test_error_argument() {
        let err = Error::Argument("test arg".to_string());
        assert_eq!(err.to_string(), "Argument error. test arg");
    }

    #[test]
    fn test_error_parse() {
        let err = Error::Parse("parse error".to_string());
        assert_eq!(err.to_string(), "Parse error. parse error");
    }

    #[test]
    fn test_error_missing() {
        let err = Error::Missing("test".to_string());
        assert_eq!(err.to_string(), "test not found.");
    }

    #[test]
    fn test_error_invalid_operation() {
        let err = Error::InvalidOperation("test op".to_string());
        assert_eq!(err.to_string(), "Invalid operation. test op");
    }

    #[test]
    fn test_error_from_std_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::from_std_error(io_err);
        
        match err {
            Error::Error(_) => {},
            _ => panic!("Should be Error variant"),
        }
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let err: Error = io_err.into();
        
        match err {
            Error::IO(_) => {},
            _ => panic!("Should be IO variant"),
        }
    }

    #[test]
    fn test_error_from_other_error() {
        let err = Error::from_other_error("test error".to_string());
        
        match err {
            Error::Other(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Should be Other variant"),
        }
    }

    #[test]
    fn test_error_queue_started() {
        let err = Error::QueueStarted;
        assert_eq!(err.to_string(), "Queue already started");
    }

    #[test]
    fn test_error_queue_completed() {
        let err = Error::QueueCompleted;
        assert_eq!(err.to_string(), "Queue already completed");
    }

    #[test]
    fn test_error_invalid_directory() {
        let err = Error::InvalidDirectory("test dir".to_string());
        assert_eq!(err.to_string(), "Invalid directory. test dir");
    }

    #[test]
    fn test_error_invalid_file() {
        let err = Error::InvalidFile("test file".to_string());
        assert_eq!(err.to_string(), "Invalid file. test file");
    }

    #[test]
    fn test_error_session() {
        let err = Error::Session("session error".to_string());
        assert_eq!(err.to_string(), "Session error. session error");
    }

    #[test]
    fn test_error_http() {
        let err = Error::Http("http error".to_string());
        assert_eq!(err.to_string(), "Http error. http error");
    }

    #[test]
    fn test_error_network() {
        let err = Error::Network("network error".to_string());
        assert_eq!(err.to_string(), "Network error. network error");
    }

    #[test]
    fn test_error_command() {
        let err = Error::Command(1, "command failed".to_string());
        assert_eq!(err.to_string(), "Command error [1] command failed");
    }

    #[test]
    fn test_error_exceeded() {
        let err = Error::Exceeded("limit exceeded".to_string());
        assert_eq!(err.to_string(), "Limit exceeded. limit exceeded");
    }

    #[test]
    fn test_error_exit_code() {
        let err = Error::ExitCode(42);
        assert_eq!(err.to_string(), "Application exited with error 42");
    }

    #[test]
    fn test_error_openai() {
        let err = Error::OpenAI("openai error".to_string());
        assert_eq!(err.to_string(), "OpenAI error. openai error");
    }

    #[test]
    fn test_error_llama() {
        let err = Error::Llama("llama error".to_string());
        assert_eq!(err.to_string(), "Llama error. llama error");
    }

    #[test]
    fn test_error_poisoned() {
        let err = Error::Poisoned("mutex poisoned".to_string());
        assert_eq!(err.to_string(), "Guard was poisoned. mutex poisoned");
    }

    #[test]
    fn test_error_from_poison_error() {
        use std::sync::{Arc, Mutex};
        
        let mutex = Arc::new(Mutex::new(42));
        let mutex_clone = Arc::clone(&mutex);
        
        // Poison the mutex by panicking in another thread
        let handle = std::thread::spawn(move || {
            let _guard = mutex_clone.lock().unwrap();
            panic!("Poison the mutex");
        });
        let _ = handle.join();
        
        // Now try to lock the poisoned mutex
        match mutex.lock() {
            Ok(_) => panic!("Mutex should be poisoned"),
            Err(poison_err) => {
                let error = Error::from_poison_error(poison_err);
                match error {
                    Error::Poisoned(_) => {},
                    _ => panic!("Should be Poisoned variant"),
                }
            }
        }
    }

    #[test]
    fn test_error_handle_poison_error_ok() {
        use std::sync::Mutex;
        
        let mutex = Mutex::new(42);
        let result = Error::handle_poison_error(mutex.lock());
        
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    fn test_error_handle_poison_error_err() {
        use std::sync::{Arc, Mutex};
        
        let mutex = Arc::new(Mutex::new(42));
        let mutex_clone = Arc::clone(&mutex);
        
        // Poison the mutex
        let handle = std::thread::spawn(move || {
            let _guard = mutex_clone.lock().unwrap();
            panic!("Poison the mutex");
        });
        let _ = handle.join();
        
        // Now try to lock the poisoned mutex
        let result = Error::handle_poison_error(mutex.lock());
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Poisoned(_) => {},
            _ => panic!("Should be Poisoned variant"),
        }
    }
}

