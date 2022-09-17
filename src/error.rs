use thiserror::Error;

pub type Result<T> = std::result::Result<T, BenchError>;

#[derive(Error, Debug)]
pub enum BenchError {
    /// benchmarking related errors
    #[error("{0}")]
    Unknown(String),
    /// app not found
    #[error("{0}")]
    AppNotFound(String),
    /// IO error
    #[error("{0}")]
    IOError(String),
    /// visualisation error
    #[error("{0}")]
    Visualization(String),
}

impl From<std::io::Error> for BenchError {
    fn from(err: std::io::Error) -> BenchError {
        BenchError::IOError(err.to_string())
    }
}
