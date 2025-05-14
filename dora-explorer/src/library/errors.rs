use thiserror::Error;


#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("Not a Directory Error: {0}")]
    NotADirectoryError(String), 

}
