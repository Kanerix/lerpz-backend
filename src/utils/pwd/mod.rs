mod error;
mod parts;
mod scheme_01;

use error::{ErrorKind, Result};
use parts::PwdParts;
use scheme_01::Scheme01;

static DEFAULT_SCHEME: &str = "01";

pub trait Scheme {
	fn hash(pwd: PwdParts) -> Result<String>;
	fn verify(pwd: &[u8]) -> Result<bool>;
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
	match scheme_name {
		"01" => Ok(Scheme01),
		_ => Err(ErrorKind::SchemeNotFound(scheme_name.into())),
	}
}

pub async fn hash_pwd<'a>(pwd: &'a str) -> Result<String> {
	let pwd_parts = PwdParts::new(pwd);
	let scheme = get_scheme(DEFAULT_SCHEME)?;
	tokio::task::spawn_blocking(move || scheme.hash(pwd_parts))
		.await
		.map_err(|_| ErrorKind::FailSpawnBlockForHash)?
}

pub async fn verify_pwd() -> bool {
	return true;
}
