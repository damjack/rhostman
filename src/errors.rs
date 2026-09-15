use thiserror::Error;

pub type RhostmanResult<T> = Result<T, RhostmanError>;

#[derive(Debug, Error)]
pub enum RhostmanError {
    #[error("generic error: {0}")]
    GenericError(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerializationError(#[from] serde_json::error::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("no such tracked source: {0}")]
    SourceNotFound(String),
    #[error("tracked source already exists: {0}")]
    SourceExists(String),
}
