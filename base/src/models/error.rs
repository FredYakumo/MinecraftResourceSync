use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error")]
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

impl From<Error> for std::io::Error {
    fn from(err: Error) -> std::io::Error {
        match err {
            Error::Io(e) => e,
            Error::StringError(e) => io::Error::new(io::ErrorKind::Other, e),
            Error::StaticStrError(e) => io::Error::new(io::ErrorKind::Other, e),
            Error::Http(e) => io::Error::new(io::ErrorKind::Other, e.to_string()),
            Error::Yaml(e) => io::Error::new(io::ErrorKind::Other, e.to_string()),
        }
    }
}