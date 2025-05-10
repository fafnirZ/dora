use thiserror::Error;

pub type DoraResults<T> = std::result::Result<T, DoraErrors>;

#[derive(Debug, Error)]
pub enum DoraErrors {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO Error: {0}")]
    IOError(String),

    #[error("CommandError: {0}")]
    CommandError(String),
}
