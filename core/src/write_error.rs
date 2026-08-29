use png::EncodingError;
use std::io::Error as IoError;
use thiserror::Error;

pub type WriteResult = Result<(), WriteError>;

#[derive(Debug, Error)]
pub enum WriteError {
    #[error(transparent)]
    Encoding(#[from] EncodingError),
    #[error(transparent)]
    Io(#[from] IoError),
}
