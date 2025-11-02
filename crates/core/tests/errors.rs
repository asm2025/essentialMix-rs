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
}

