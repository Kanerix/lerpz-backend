use argon2::{
	password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
	PasswordVerifier, Version,
};
use lazy_static::lazy_static;

use crate::config::CONFIG;

use super::{
	error::{Error, Result},
	Scheme,
};

lazy_static! {
	static ref ARGON2: Argon2<'static> = {
		Argon2::new_with_secret(
			&CONFIG.PWD_SECRET.as_bytes(),
			Algorithm::Argon2id,
			Version::V0x13,
			Params::default(),
		)
		.unwrap()
	};
}

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(&self, pwd: &str, salt: &str) -> Result<String> {
		let salt = SaltString::encode_b64(salt.as_bytes()).map_err(|_| Error::PwdSalt)?;

		let pwd = ARGON2
			.hash_password(pwd.as_bytes(), &salt)
			.map_err(|_| Error::PwdHash)?
			.to_string();

		Ok(pwd)
	}

	fn validate(&self, pwd_hash: &str, pwd_ref: &str, _: Option<&str>) -> Result<bool> {
		let pwd_hash_parsed = PasswordHash::new(pwd_hash)
			.inspect_err(|err| {
				println!("{err}");
			})
			.map_err(|_| Error::PwdHash)?;

		let pwd_ref_bytes = pwd_ref.as_bytes();

		Ok(ARGON2
			.verify_password(pwd_ref_bytes, &pwd_hash_parsed)
			.is_ok())
	}
}
