use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operation is not supported")]
    NotSupported,

    #[error("Not implemented")]
    NotImplemented,

    #[error("Operation is canceled")]
    Canceled,

    #[error("Operation timed out")]
    Timeout,

    #[error("Item not found. {0}")]
    NotFound(String),

    #[error("No input was provided.")]
    NoInput,

    #[error("Invalid input. {0}")]
    InvalidInput(String),

    #[error("Argument error. {0}")]
    Argument(String),

    #[error("Index out of range.")]
    IndexOutOfRange,

    #[error("Not enough data.")]
    NotEnoughData,

    #[error("Parse error. {0}")]
    Parse(String),

    #[error("{0} not found.")]
    Missing(String),

    #[error("Invalid operation. {0}")]
    InvalidOperation(String),

    #[error("Invalid directory. {0}")]
    InvalidDirectory(String),

    #[error("Invalid file. {0}")]
    InvalidFile(String),

    #[error("Queue already started")]
    QueueStarted,

    #[error("Queue already completed")]
    QueueCompleted,

    #[error("Guard was poisoned. {0}")]
    Poisoned(String),

    #[error("Session error. {0}")]
    Session(String),

    #[error("Http error. {0}")]
    Http(String),

    #[error("Network error. {0}")]
    Network(String),

    #[error("Command error [{0}] {1}")]
    Command(i32, String),

    #[error("Limit exceeded. {0}")]
    Exceeded(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Application exited with error {0}")]
    ExitCode(i32),

    #[error("OpenAI error. {0}")]
    OpenAI(String),

    #[error("Llama error. {0}")]
    Llama(String),

    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    Error(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// Create an error from a standard error. This is the preferred way to create an error from an error that can be sent between threads.
    pub fn from_std_error<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Error::Error(Box::new(err))
    }

    /// Create an error from a PoisonError, preserving the original error information
    /// and reclaiming the poisoned guard.
    pub fn from_poison_error<T: std::fmt::Debug>(err: std::sync::PoisonError<T>) -> Self {
        let guard_info = format!("{:?}", err.get_ref());
        let _poisoned_guard = err.into_inner(); // Reclaim the guard
        Error::Poisoned(format!("Guard was poisoned. {}", guard_info))
    }

    /// Helper function to handle poisoned mutex errors.
    /// Reclaims the poisoned guard and converts the error to an Error::Poisoned variant.
    pub fn handle_poison_error<T, E: std::fmt::Debug>(
        result: std::result::Result<T, std::sync::PoisonError<E>>,
    ) -> crate::Result<T> {
        match result {
            Ok(value) => Ok(value),
            Err(e) => {
                let error_msg = format!("{}", e);
                let _poisoned_guard = e.into_inner(); // Reclaim the guard
                Err(Error::Poisoned(if error_msg.is_empty() {
                    "Poison error".to_string()
                } else {
                    error_msg
                }))
            }
        }
    }
}
