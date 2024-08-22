use std::io::{self, ErrorKind};

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

impl From<Error> for io::Error {
    fn from(error: Error) -> io::Error {
        match error {
            Error::Io(err) => err, // Directly return the io::Error
            Error::StringError(msg) => io::Error::new(ErrorKind::Other, msg),
            Error::StaticStrError(s) => io::Error::new(ErrorKind::Other, s),
            Error::Http(err) => io::Error::new(ErrorKind::Other, format!("HTTP error: {}", err)),
            Error::Yaml(err) => io::Error::new(ErrorKind::Other, format!("YAML error: {}", err)),
        }
    }
}