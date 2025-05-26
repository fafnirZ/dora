use thiserror::Error;


#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("Not a Directory Error: {0}")]
    NotADirectoryError(String), 
    
    #[error("Not a LocalPath Error: {0}")]
    NotALocalPath(String),
    
    #[error("Not a RemotePath Error: {0}")]
    NotARemotePath(String),
}
