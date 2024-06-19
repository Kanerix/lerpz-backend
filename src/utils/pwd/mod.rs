mod error;
mod parts;
mod scheme;

use error::{Error, Result};
use parts::PwdParts;
use scheme::{get_scheme, Scheme};

static LATEST_SCHEME: &'static str = "01";

/// This hashes a password using the latest scheme.
///
/// # Safety
///
/// This uses the latest scheme, because we use the [`PwdParts::new`] method.
pub async fn hash_pwd(pwd: String, salt: String) -> Result<String> {
	unsafe { hash_pwd_parts(PwdParts::new(pwd, salt)).await }
}

/// This hashes a password using custom PwdParts.
///
/// This function can create a password using an old scheme. This is
/// why it is unsafe to call. You can use this function together with
/// the [`PwdParts::new`] method to create a password using the latest
/// scheme.
pub async unsafe fn hash_pwd_parts(raw_parts: PwdParts) -> Result<String> {
	let scheme = get_scheme(&raw_parts.scheme_name())?;
	tokio::task::spawn_blocking(move || {
		scheme
			.hash(raw_parts.pwd(), raw_parts.salt())
			.map_err(|err| Error::SchemeError(err))
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForHash)
	.and_then(|res| res)
}
