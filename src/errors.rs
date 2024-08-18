use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
