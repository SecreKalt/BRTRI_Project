use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("PCL error: {0}")]
    Pcl(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Processing error: {0}")]
    Processing(String),
}

impl From<pcl::Error> for Error {
    fn from(err: pcl::Error) -> Self {
        Error::Pcl(err.to_string())
    }
}
