/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `scheme` module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("scheme with name \"{0}\" does not exist")]
	SchemeNotFound(String),
	#[error("failed parsing password")]
	PwdParsingFailed,
	#[error("error creating password salt")]
	PwdSalt,
	#[error("error creating password hash")]
	PwdHash,
}
