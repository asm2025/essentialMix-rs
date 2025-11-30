#[cfg(test)]
mod tests {
    use emixcore::Error;

    #[test]
    fn test_error_from_std_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::from_std_error(io_err);

        match err {
            Error::Error(_) => {}
            _ => panic!("Should be Error variant"),
        }
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let err: Error = io_err.into();

        match err {
            Error::IO(_) => {}
            _ => panic!("Should be IO variant"),
        }
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
                    Error::Poisoned(_) => {}
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
            Error::Poisoned(_) => {}
            _ => panic!("Should be Poisoned variant"),
        }
    }
}
