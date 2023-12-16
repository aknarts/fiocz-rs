use thiserror::Error;

#[derive(Error, Debug)]
pub enum FioError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("We have hit the API limit, try again later")]
    Limit,
    #[error("The token does not exist or is deactivated")]
    Token,
    #[error("The request was malformed")]
    Malformed,
    #[error("The requesting too many items")]
    TooLarge,
    #[error("Invalid date format")]
    InvalidDateFormat,
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Missing body")]
    MissingBody,
}

