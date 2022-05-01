use thiserror::Error;

pub type RhostmanResult<T> = Result<T, RhostmanError>;

#[derive(Debug, Error)]
pub enum RhostmanError {
    #[error("errore generico: {:?}", .0)]
    GenericError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_json::error::Error),
}
