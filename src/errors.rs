use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid home path")]
    InvalidHomePath,

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Mod meta.cpp missing or unreadable for mod ID {0}")]
    MissingMeta(u64),

    #[error("Path conversion error: {0}")]
    PathConversionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Unsupported platform")]
    UnsupportedPlatform,
}
