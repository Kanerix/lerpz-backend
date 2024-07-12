pub mod error;
pub mod parts;
pub mod scheme;

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

/// Validates a hash against a password and salt.
///
/// The hash needs to be parseable to [`HashParts`]. See [`HashParts::from_str`] to see
/// how the format works.
pub async fn validate_pwd(
	pwd_hash: String,
	pwd_ref: String,
	pwd_ref_salt: Option<String>,
) -> Result<bool> {
	unsafe { validate_pwd_parts(HashParts::from_str(&pwd_hash)?, pwd_ref, pwd_ref_salt).await }
}

/// Validates a password using [`HashParts`] and a password reference with a optional salt.
///
/// This function validates a password hash against a password and optional salt. This uses the
/// [`HashParts`] to decide wich scheme to use for validating. You can use the [`HashParts::from_str`]
/// to create the [`HashParts`] needed for validating the password scheme. If you do not use the correct
/// scheme, this function will error.
///
/// # Safety
///
/// Make sure you use [`HashParts::from_str`] to get the scheme or be certain that the scheme given is
/// the same as what was used to create the password hash.
pub async unsafe fn validate_pwd_parts(
	hash_parts: HashParts,
	pwd_ref: String,
	pwd_ref_salt: Option<String>,
) -> Result<bool> {
	let scheme = get_scheme(&hash_parts.scheme_name)?;
	tokio::task::spawn_blocking(move || {
		scheme
			.validate(
				&hash_parts.hash,
				&pwd_ref,
				pwd_ref_salt.as_ref().map(|x| x.as_str()),
			)
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

		let salt = uuid::Uuid::new_v4().to_string();
		let hash = hash_pwd("password".to_string(), salt.clone())
			.await
			.unwrap();

		let pwd_wrong = validate_pwd(hash.clone(), "not_password".to_string(), Some(salt.clone()))
			.await
			.unwrap();
		let pwd_correct = validate_pwd(hash.clone(), "password".to_string(), Some(salt.clone()))
			.await
			.unwrap();

		assert_eq!(pwd_wrong, false);
		assert_eq!(pwd_correct, true);
	}
}
