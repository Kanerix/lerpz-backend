/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `scheme` module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("no scheme named \"{0}\" exist")]
	SchemeNotFound(String),
	#[error("error creating password salt")]
	PwdSalt,
	#[error("error creating password hash")]
	PwdHash,
}
