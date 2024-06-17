use super::scheme;

/// A type alias for [`Result<T, ErrorKind>`].
///
/// Used by this module to return the same error kind for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different error kinds the `pwd` module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("failed parsing scheme for password")]
	PwdWithSchemeFailedParse,
	#[error("failed spawning thread for validation")]
	FailSpawnBlockForValidate,
	#[error("failed spawning thread for hashing")]
	FailSpawnBlockForHash,
	#[error("no scheme named \"{0}\" found")]
	SchemeError(#[from] scheme::error::Error),
	#[error("error creating password salt")]
	PasswordSalt,
	#[error("error creating password hash")]
	PasswordHash,
}
