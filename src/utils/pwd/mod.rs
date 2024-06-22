mod error;
mod parts;
mod scheme;

use std::str::FromStr;

use error::{Error, Result};
use parts::{HashParts, PwdParts};
use scheme::{get_scheme, Scheme};

static LATEST_SCHEME: &'static str = "01";

/// Hashes a password using the latest scheme.
///
/// # Safety
///
/// This uses the latest scheme, because we use the [`PwdParts::new`] method.
pub async fn hash_pwd(pwd: String, salt: String) -> Result<String> {
	unsafe { hash_pwd_parts(PwdParts::new(pwd, salt)).await }
}

/// Hashes a password using custom [`PwdParts`].
///
/// This function can create a password using an old scheme. This is
/// why it is unsafe to call. You can use this function together with
/// the [`PwdParts::new`] method to create a password using the latest
/// scheme.
pub async unsafe fn hash_pwd_parts(pwd_parts: PwdParts) -> Result<String> {
	let scheme = get_scheme(&pwd_parts.scheme_name)?;
	tokio::task::spawn_blocking(move || {
		scheme
			.hash(&pwd_parts.pwd, &pwd_parts.salt)
			.and_then(|hash| Ok(format!("#{}#{}", pwd_parts.scheme_name, hash)))
			.map_err(|err| Error::SchemeError(err))
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForHash)
	.and_then(|res| res)
}

/// Validates a password hash against a real password.
///
/// The password hash needs to be parseable to [`PwdParts`].
/// See [`PwdParts::from_str`] to see how the format works.
pub async fn validate_pwd(pwd_hash: String, pwd_ref: String) -> Result<bool> {
	validate_pwd_parts(HashParts::from_str(&pwd_hash)?, pwd_ref).await
}

/// Validates a password using custom [`PwdParts`] and a password reference.
///
/// This function validates a password hash against a password. This uses the
/// [`PwdParts`] to decide wich password scheme to use. You can use the [`PwdParts::from_str`]
/// to create the [`PwdParts`] needed for validating the password scheme.
pub async fn validate_pwd_parts(hash_parts: HashParts, pwd_ref: String) -> Result<bool> {
	let scheme = get_scheme(&hash_parts.scheme_name)?;
	tokio::task::spawn_blocking(move || {
		scheme
			.validate(&hash_parts.hash, &pwd_ref)
			.map_err(|err| Error::SchemeError(err))
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForValidate)
	.and_then(|res| res)
}

#[cfg(test)]
mod tests {
	use super::{hash_pwd, validate_pwd};

	#[tokio::test]
	async fn test_password_hashing_and_validate() {
		dotenv::dotenv().unwrap();

		let hash = hash_pwd("password".to_string(), uuid::Uuid::new_v4().to_string())
			.await
			.unwrap();

		let pwd_wrong = validate_pwd(hash.clone(), "not_password".to_string())
			.await
			.unwrap();
		let pwd_correct = validate_pwd(hash.clone(), "password".to_string())
			.await
			.unwrap();

		assert_eq!(pwd_wrong, false);
		assert_eq!(pwd_correct, true);
	}
}
