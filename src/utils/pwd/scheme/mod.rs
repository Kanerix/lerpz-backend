use error::{Error, Result};
use scheme_01::Scheme01;

pub mod error;
pub mod scheme_01;

/// Implemented by schemes that can hash and validate passwords.
pub trait Scheme {
	/// This function hashes a password from some [`PwdParts`](crate::utils::pwd::parts::PwdParts).
	fn hash(&self, pwd: &str, salt: &str) -> Result<String>;
	/// This function validates a password against some other password.
	fn validate(&self, pwd_hash: &str, pwd_ref: &str, pwd_ref_salt: Option<&str>) -> Result<bool>;
}

/// Returns a scheme given a scheme name as a string.
///
/// # Errors
///
/// If the scheme given does not exist.
pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
	match scheme_name {
		"01" => Ok(Scheme01),
		_ => Err(Error::SchemeNotFound(scheme_name.into())),
	}
}
