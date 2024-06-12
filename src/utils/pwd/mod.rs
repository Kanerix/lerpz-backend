mod error;
mod scheme_01;

use error::Result;

static DEFAULT_SCHEME: &str = "01";

trait Scheme {
	fn hash(pwd: &[u8]) -> Result<&str>;
	fn verify(pwd: &[u8]) -> bool;
}

struct PwdParts<'a, 'b> {
	scheme: &'a str,
	pwd: &'b str,
}

impl<'a, 'b> PwdParts<'a, 'b> {
	pub fn new(pwd: &'b str) -> PwdParts<'a, 'b> {
		return PwdParts {
			scheme: DEFAULT_SCHEME,
			pwd,
		};
	}
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
	match scheme_name {
		"01" => Scheme01,
		_ => Err(P),
	}
}

pub async fn hash_pwd<'a, 'b>(pwd: PwdParts<'a, 'b>) -> String {
	let scheme = scheme_01::Scheme01;
	tokio::task::spawn_blocking(move || Scheme::hash(pwd.into()));
	return "test".into();
}

pub async fn verify_pwd() -> bool {
	return true;
}
