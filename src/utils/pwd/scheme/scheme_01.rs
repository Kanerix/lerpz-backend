use std::sync::OnceLock;

use argon2::{
	password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
	PasswordVerifier, Version,
};

use crate::config::web_config;

use super::{
	error::{Error, Result},
	Scheme,
};

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(&self, pwd: &str, salt: &str) -> Result<String> {
		let argon2 = get_argon2();

		let salt = SaltString::encode_b64(salt.as_bytes()).map_err(|_| Error::PwdSalt)?;

		let pwd = argon2
			.hash_password(pwd.as_bytes(), &salt)
			.map_err(|_| Error::PwdHash)?
			.to_string();

		Ok(pwd)
	}

	fn validate(&self, pwd_hash: &str, pwd_ref: &str, _: &str) -> Result<bool> {
		let argon2 = get_argon2();

		let pwd_hash_parsed = PasswordHash::new(pwd_hash)
			.inspect_err(|err| {
				println!("{err}");
			})
			.map_err(|_| Error::PwdHash)?;

		let pwd_ref_bytes = pwd_ref.as_bytes();

		Ok(argon2
			.verify_password(pwd_ref_bytes, &pwd_hash_parsed)
			.is_ok())
	}
}

fn get_argon2() -> &'static Argon2<'static> {
	static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Argon2::new_with_secret(
			&web_config().PWD_SECRET.as_bytes(),
			Algorithm::Argon2id,
			Version::V0x13,
			Params::default(),
		)
		.unwrap()
	})
}
