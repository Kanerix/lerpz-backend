/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `pwd` module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("jwt error: {0}")]
	TokenError(#[from] jsonwebtoken::errors::Error),
}
