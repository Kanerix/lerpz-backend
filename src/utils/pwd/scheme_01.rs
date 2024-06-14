use std::sync::OnceLock;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use uuid::Uuid;

use super::{
	error::{Error, Result},
	parts::{PwdParts, PwdPartsRaw},
	Scheme,
};

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(&self, pwd_parts: PwdPartsRaw) -> Result<PwdParts> {
		let argon2 = get_argon2();

		let salt =
			SaltString::encode_b64(Uuid::new_v4().as_bytes()).map_err(|_| Error::PasswordSalt)?;

		let pwd = argon2
			.hash_password(pwd_parts.pwd().as_bytes(), &salt)
			.map_err(|_| Error::PasswordSalt)?
			.to_string();

		Ok(PwdParts::new(pwd))
	}

	fn validate(&self, pwd_parts: PwdParts, pwd_ref: &str) -> Result<bool> {
		let argon2 = get_argon2();

		let pwd_ref_hash = PasswordHash::new(pwd_ref).map_err(|_| Error::PasswordHash)?;

		let hash_bytes = pwd_parts.hash().as_bytes();

		Ok(argon2.verify_password(hash_bytes, &pwd_ref_hash).is_ok())
	}
}

fn get_argon2() -> &'static Argon2<'static> {
	static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

	INSTANCE.get_or_init(|| Argon2::default())
}
