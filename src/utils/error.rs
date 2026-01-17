use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiffError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Binary file cannot be compared as text: {0}")]
    BinaryFile(String),

    #[error("Invalid encoding: {0}")]
    EncodingError(String),

    #[error("Web server error: {0}")]
    WebServerError(String),
}

pub type Result<T> = std::result::Result<T, DiffError>;
