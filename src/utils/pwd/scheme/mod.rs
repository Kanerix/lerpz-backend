use super::parts::{PwdParts, PwdPartsHashed};
use error::{Error, Result};
use scheme_01::Scheme01;

pub mod error;
pub mod scheme_01;

pub trait Scheme {
	/// This function hashes a password from some [`PwdParts`].
	fn hash(&self, pwd: PwdParts) -> Result<PwdPartsHashed>;
	/// This function validates a password against some other password.
	fn validate(&self, pwd: PwdPartsHashed, pwd_ref: &str) -> Result<bool>;
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
	match scheme_name {
		"01" => Ok(Scheme01),
		_ => Err(Error::SchemeNotFound(scheme_name.into())),
	}
}
