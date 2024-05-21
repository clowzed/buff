use tokio::io;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
