pub type Result<T> = std::result::Result<T, ErrorKind>;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
	#[error("failed parsing scheme for password")]
	PwdWithSchemeFailedParse,
	#[error("failed spawning thread for validation")]
	FailSpawnBlockForValidate,
	#[error("failed spawning thread for hashing")]
	FailSpawnBlockForHash,
	#[error("no scheme named \"{0}\" found")]
	SchemeNotFound(String),
}
