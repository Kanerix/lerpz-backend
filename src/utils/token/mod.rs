pub mod claims;
pub mod error;
mod keys;

use claims::{TokenClaims, TokenUser};
use error::{Error, Result};
use jsonwebtoken::{decode, encode, TokenData};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_access_token(user: impl Into<TokenUser>) -> Result<String> {
	encode(
		&jsonwebtoken::Header::default(),
		&TokenClaims::new(user),
		&keys::JWT_ENCODE_KEY,
	)
	.map_err(|err| Error::TokenError(err))
}

pub fn decode_access_token(token: &str) -> Result<TokenData<TokenClaims>> {
	decode(
		token,
		&keys::JWT_DECODE_KEY,
		&jsonwebtoken::Validation::default(),
	)
	.map_err(|err| Error::TokenError(err))
}

pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}
