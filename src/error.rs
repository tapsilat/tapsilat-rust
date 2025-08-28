use std::fmt;

#[derive(Debug)]
pub enum TapsilatError {
    Http(ureq::Error),
    Serialization(std::io::Error),
    InvalidResponse(String),
    ApiError { status_code: u16, message: String },
    ConfigError(String),
}

impl fmt::Display for TapsilatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TapsilatError::Http(err) => write!(f, "HTTP error: {}", err),
            TapsilatError::Serialization(err) => write!(f, "Serialization error: {}", err),
            TapsilatError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            TapsilatError::ApiError {
                status_code,
                message,
            } => {
                write!(f, "API error ({}): {}", status_code, message)
            }
            TapsilatError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for TapsilatError {}

impl From<ureq::Error> for TapsilatError {
    fn from(err: ureq::Error) -> Self {
        TapsilatError::Http(err)
    }
}

impl From<std::io::Error> for TapsilatError {
    fn from(err: std::io::Error) -> Self {
        TapsilatError::Serialization(err)
    }
}

pub type Result<T> = std::result::Result<T, TapsilatError>;
