mod error;
mod parts;
mod scheme_01;

use error::{Error, Result};
use parts::{PwdParts, PwdPartsRaw};
use scheme_01::Scheme01;

static DEFAULT_SCHEME: &'static str = "01";

pub trait Scheme {
	fn hash(&self, pwd: PwdPartsRaw) -> Result<PwdParts>;
	fn validate(&self, pwd_parts: PwdParts, pwd: &str) -> Result<bool>;
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
	match scheme_name {
		"01" => Ok(Scheme01),
		_ => Err(Error::SchemeNotFound(scheme_name.into())),
	}
}

pub async fn hash_pwd(pwd: String) -> Result<PwdParts> {
	hash_pwd_parts(PwdPartsRaw::new(pwd)).await
}

pub async fn hash_pwd_parts<'a>(raw_parts: PwdPartsRaw) -> Result<PwdParts> {
	let scheme = get_scheme(&raw_parts.scheme_name())?;
	tokio::task::spawn_blocking(move || scheme.hash(raw_parts))
		.await
		.map_err(|_| Error::FailSpawnBlockForHash)
		.and_then(|result| result)
}
