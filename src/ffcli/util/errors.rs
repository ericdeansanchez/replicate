//! Primary error structures for ffcli.
use std::io;

/// Error types for ffcli.
#[derive(Debug)]
pub enum FfcliError {
    Io(io::Error),
}

impl From<io::Error> for FfcliError {
    fn from(err: io::Error) -> FfcliError {
        FfcliError::Io(err)
    }
}

/// Custom result type for ffcli.
pub type Result<T> = std::result::Result<T, FfcliError>;
