//! Primary error structures for replicate.
use std::io;

/// Error types for replicate.
#[derive(Debug)]
pub enum ReplicateError {
    Io(io::Error),
}

impl From<io::Error> for ReplicateError {
    fn from(err: io::Error) -> Self {
        ReplicateError::Io(err)
    }
}

/// Custom result type for replicate.
pub type Result<T> = std::result::Result<T, ReplicateError>;
