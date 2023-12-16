//! Error types
use thiserror::Error;

/// Error types
#[derive(Error, Debug)]
pub enum FioError {
    /// Reqwest error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Serde JSON error
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// API limit reached
    #[error("We have hit the API limit, try again later")]
    Limit,
    /// Invalid token
    #[error("The token does not exist or is deactivated")]
    Token,
    /// Malformed request
    #[error("The request was malformed")]
    Malformed,
    /// Too large request
    #[error("The requesting too many items")]
    TooLarge,
    /// Invalid date format
    #[error("Invalid date format")]
    InvalidDateFormat,
    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    /// Missing body
    #[error("Missing body")]
    MissingBody,
}

