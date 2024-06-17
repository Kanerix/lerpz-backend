use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use super::{error::Error, LATEST_SCHEME};

lazy_static! {
	/// A Regular expression used when parsing the password string.
	static ref PWD_PARTS_REGEX: Regex =
		Regex::new(r"^#(?<scheme_name>\d{2})#(?<hash>\w.+)$").unwrap();
}

/// This is the parts that every password needs to be created.
pub struct PwdParts {
	scheme_name: String,
	pwd: String,
}

/// This is the parts that every password needs when hashed.
pub struct PwdPartsHashed {
	scheme_name: String,
	hash: String,
}

impl PwdParts {
	/// Creates a new [`PwdParts`] structure. This has
	pub fn new(pwd: String) -> Self {
		Self {
			scheme_name: LATEST_SCHEME.into(),
			pwd,
		}
	}

	/// Gets the name of the scheme used.
	pub fn scheme_name(&self) -> &String {
		&self.scheme_name
	}

	/// Gets the password used.
	pub fn pwd(&self) -> &String {
		&self.pwd
	}

	/// This sets the scheme used to hash the password.
	///
	/// This is unsafe to call becuase you might create an password with
	/// an old and unsafe hashing scheme. You should always use the latest if possible.
	unsafe fn with_scheme(mut self, scheme_name: String) -> Self {
		self.scheme_name = scheme_name;
		self
	}
}

impl PwdPartsHashed {
	pub fn new(hash: String) -> Self {
		Self {
			scheme_name: LATEST_SCHEME.into(),
			hash,
		}
	}

	pub fn scheme_name(&self) -> &String {
		&self.scheme_name
	}

	pub fn hash(&self) -> &String {
		&self.hash
	}

	pub fn into_hash_string(self) -> String {
		format!("#{}#{}", self.scheme_name, self.hash)
	}
}

impl FromStr for PwdPartsHashed {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let captures = PWD_PARTS_REGEX
			.captures(s)
			.ok_or(Error::PwdWithSchemeFailedParse)?;

		let scheme_name = captures
			.name("scheme_name")
			.ok_or(Error::PwdWithSchemeFailedParse)?
			.as_str()
			.to_string();
		let hash = captures
			.name("pwd")
			.ok_or(Error::PwdWithSchemeFailedParse)?
			.as_str()
			.to_string();

		Ok(PwdPartsHashed { scheme_name, hash })
	}
}
