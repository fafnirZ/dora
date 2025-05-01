use thiserror::Error;

pub type DoraResults<T> = std::result::Result<T, DoraErrors>;

#[derive(Debug, Error)]
pub enum DoraErrors {
    #[error("File not found: {0}")]
    FileNotFound(String),
}
