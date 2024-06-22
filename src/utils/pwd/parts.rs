use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use super::{error::Error, LATEST_SCHEME};

/// This is the parts that every password needs to be created.
pub struct PwdParts {
	pub scheme_name: String,
	pub salt: String,
	pub pwd: String,
}

/// This is the parts that every password needs to be created.
#[derive(Debug)]
pub struct HashParts {
	pub scheme_name: String,
	pub hash: String,
}

impl PwdParts {
	/// Creates a new [`PwdParts`] structure.
	///
	/// This will have the latest scheme for hashing.
	pub fn new(pwd: String, salt: String) -> Self {
		Self {
			scheme_name: LATEST_SCHEME.into(),
			salt,
			pwd,
		}
	}
}

impl HashParts {
	/// Creates a new [`HashParts`] structure.
	pub fn new(scheme_name: String, hash: String) -> Self {
		Self { scheme_name, hash }
	}
}

lazy_static! {
	static ref PWD_PARTS_REGEX: Regex = Regex::new(r"^#(?<scheme_name>\w+)#(?<hash>.+)$").unwrap();
}

impl FromStr for HashParts {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let captures = PWD_PARTS_REGEX.captures(s).ok_or(Error::PwdParsingFailed)?;

		let scheme_name = captures
			.name("scheme_name")
			.ok_or(Error::PwdParsingFailed)?
			.as_str()
			.to_string();
		let hash = captures
			.name("hash")
			.ok_or(Error::PwdParsingFailed)?
			.as_str()
			.to_string();

		Ok(HashParts::new(scheme_name, hash))
	}
}
