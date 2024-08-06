use super::scheme;

/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `pwd` module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("failed spawning thread for validation")]
	FailSpawnBlockForValidate,
	#[error("failed spawning thread for hashing")]
	FailSpawnBlockForHash,
	#[error("failed parsing password: {0}")]
	PwdParsingFailed(String),
	#[error("scheme error: {0}")]
	SchemeError(#[from] scheme::error::Error),
}
