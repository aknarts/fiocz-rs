use thiserror::Error;

#[derive(Error, Debug)]
pub enum FioError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error)
}

