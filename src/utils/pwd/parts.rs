use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use super::{error::Error, DEFAULT_SCHEME};

pub struct PwdPartsRaw {
	scheme_name: String,
	pwd: String,
}

pub struct PwdParts {
	scheme_name: String,
	hash: String,
}

impl PwdPartsRaw {
	pub fn new(pwd: String) -> PwdPartsRaw {
		PwdPartsRaw {
			scheme_name: DEFAULT_SCHEME.into(),
			pwd,
		}
	}

	pub fn scheme_name(&self) -> &String {
		&self.scheme_name
	}

	pub fn pwd(&self) -> &String {
		&self.pwd
	}

	pub fn into_pwd_string(self) -> String {
		self.pwd
	}
}

impl PwdParts {
	pub fn new(hash: String) -> PwdParts {
		PwdParts {
			scheme_name: DEFAULT_SCHEME.into(),
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

impl FromStr for PwdParts {
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

		Ok(PwdParts { scheme_name, hash })
	}
}

lazy_static! {
	static ref PWD_PARTS_REGEX: Regex =
		Regex::new(r"^#(?<scheme_name>\d{2})#(?<hash>\w.+)$").unwrap();
}
