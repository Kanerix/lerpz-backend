use std::sync::OnceLock;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use uuid::Uuid;

use super::{
	error::{Error, Result},
	Scheme,
};
use crate::utils::pwd::parts::{PwdParts, PwdPartsHashed};

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(&self, pwd_parts: PwdParts) -> Result<PwdPartsHashed> {
		let argon2 = get_argon2();

		let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes()).map_err(|_| Error::PwdSalt)?;

		let pwd = argon2
			.hash_password(pwd_parts.pwd().as_bytes(), &salt)
			.map_err(|_| Error::PwdHash)?
			.to_string();

		Ok(PwdPartsHashed::new(pwd))
	}

	fn validate(&self, pwd_hash: PwdPartsHashed, pwd_ref: &str) -> Result<bool> {
		let argon2 = get_argon2();

		let hash_ref = PasswordHash::new(pwd_ref).map_err(|_| Error::PwdHash)?;

		let hash_bytes = pwd_hash.hash().as_bytes();

		Ok(argon2.verify_password(hash_bytes, &hash_ref).is_ok())
	}
}

fn get_argon2() -> &'static Argon2<'static> {
	static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

	INSTANCE.get_or_init(|| Argon2::default())
}
