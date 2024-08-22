#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("String error: {0}")]
    StringError(String),
    #[error("String error: {0}")]
    StaticStrError(&'static str),
    #[error("HTTP error")]
    Http(#[from] reqwest::Error),
    #[error("Serde yml error: {0}")]
    Yaml(#[from] serde_yaml::Error)
}

pub type Result<T> = std::result::Result<T, Error>;