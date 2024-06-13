use std::str::FromStr;

use super::{error::ErrorKind, DEFAULT_SCHEME};

pub struct PwdParts<'a, 'b> {
	scheme: &'a str,
	hash: &'b str,
}

impl<'a, 'b> PwdParts<'a, 'b> {
	pub fn new(hash: &'b str) -> PwdParts<'a, 'b> {
		return PwdParts {
			scheme: DEFAULT_SCHEME,
			hash,
		};
	}
}

impl<'a, 'b> FromStr for PwdParts<'a, 'b> {
	type Err = ErrorKind;

	fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
		todo!()
	}
}
