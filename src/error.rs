use thiserror::Error;

#[derive(Error, Debug)]
pub enum MorelError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File watcher error: {0}")]
    Watcher(#[from] notify::Error),

    #[error("The file was deleted")]
    FileDeleted,
}

pub type Result<T> = std::result::Result<T, MorelError>;
