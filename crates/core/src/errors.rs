use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Operation is canceled")]
    Canceled,

    #[error("Operation is not supported")]
    NotSupported,

    #[error("Not implemented")]
    NotImplemented,

    #[error("Item not found. {0}")]
    NotFound(String),

    #[error("No input was provided.")]
    NoInput,

    #[error("Invalid input. {0}")]
    Invalid(String),

    #[error("Invalid operation. {0}")]
    InvalidOperation(String),

    #[error("Invalid directory. {0}")]
    InvalidDirectory(String),

    #[error("Invalid file. {0}")]
    InvalidFile(String),

    #[error("Missing error. {0}")]
    Missing(String),

    #[error("Argument error. {0}")]
    Argument(String),

    #[error("Operation timed out")]
    Timeout,

    #[error("Operation timed out")]
    Poisoned(String),

    #[error(transparent)]
    Serialization(#[from] serde_json::Error),

    #[error("Session error. {0}")]
    Session(String),

    #[error("Http error. {0}")]
    Http(String),

    #[error("Network error. {0}")]
    Network(String),

    #[error("Command error {0}. {1}")]
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
    pub fn from_other_error(message: String) -> Self {
        Error::Other(message)
    }

    pub fn from_std_error<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Error::Error(Box::new(err))
    }
}
