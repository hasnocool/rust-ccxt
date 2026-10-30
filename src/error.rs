use thiserror::Error;

#[derive(Error, Debug)]
pub enum CcxtError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("api error ({code}): {message}")]
    Api { code: i32, message: String },
    #[error("unsupported operation for exchange: {0}")]
    Unsupported(String),
    #[error("authentication required")]
    AuthRequired,
    #[error("invalid symbol: {0}")]
    InvalidSymbol(String),
}

pub type CcxtResult<T> = Result<T, CcxtError>;
