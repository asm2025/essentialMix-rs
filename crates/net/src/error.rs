use thiserror::Error;
use emixcore::Error as CoreError;

/// Network-specific error types
#[derive(Error, Debug)]
pub enum NetError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Mail error: {0}")]
    Mail(String),

    #[error("URL parsing error: {0}")]
    UrlParse(String),

    #[error("HTML decoding error: {0}")]
    HtmlDecode(String),

    #[error("VPN error: {0}")]
    Vpn(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

impl From<NetError> for CoreError {
    fn from(err: NetError) -> Self {
        match err {
            NetError::Http(msg) => CoreError::Http(msg),
            NetError::Network(msg) => CoreError::Network(msg),
            NetError::Mail(msg) => CoreError::Other(format!("Mail error: {}", msg)),
            NetError::UrlParse(msg) => CoreError::Parse(format!("URL parsing error: {}", msg)),
            NetError::HtmlDecode(msg) => CoreError::Other(format!("HTML decoding error: {}", msg)),
            NetError::Vpn(msg) => CoreError::Other(format!("VPN error: {}", msg)),
            NetError::OperationFailed(msg) => CoreError::Other(format!("Operation failed: {}", msg)),
        }
    }
}

/// Result type alias for network operations
pub type Result<T> = std::result::Result<T, NetError>;

/// Helper functions for creating network-specific errors
impl NetError {
    pub fn http(msg: impl Into<String>) -> Self {
        NetError::Http(msg.into())
    }

    pub fn network(msg: impl Into<String>) -> Self {
        NetError::Network(msg.into())
    }

    pub fn mail(msg: impl Into<String>) -> Self {
        NetError::Mail(msg.into())
    }

    pub fn url_parse(msg: impl Into<String>) -> Self {
        NetError::UrlParse(msg.into())
    }

    pub fn html_decode(msg: impl Into<String>) -> Self {
        NetError::HtmlDecode(msg.into())
    }

    pub fn vpn(msg: impl Into<String>) -> Self {
        NetError::Vpn(msg.into())
    }
}

