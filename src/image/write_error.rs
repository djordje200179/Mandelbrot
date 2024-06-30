use std::{error::Error, fmt::{Display, Formatter, Result as FmtResult}, io};
use png::EncodingError;

pub type WriteResult = Result<(), WriteError>;

#[derive(Debug)]
pub enum WriteError {
	EncodingError(EncodingError),
	IoError(io::Error),
}

impl Error for WriteError {}

impl Display for WriteError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			WriteError::EncodingError(e) => e.fmt(f),
			WriteError::IoError(e) => e.fmt(f),
		}
	}
}

impl From<EncodingError> for WriteError {
	fn from(e: EncodingError) -> Self {
		WriteError::EncodingError(e)
	}
}

impl From<io::Error> for WriteError {
	fn from(e: io::Error) -> Self {
		WriteError::IoError(e)
	}
}