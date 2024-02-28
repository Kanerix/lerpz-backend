use jsonwebtoken::{errors::Result as EncodeResult, Algorithm, EncodingKey, Header};

use super::claims::{JwtAudience, JwtIssuer, TokenClaims, TokenUser};

pub struct AuthToken {
	header: Header,
	claims: TokenClaims,
}

impl AuthToken {
	pub fn new(user: impl Into<TokenUser>) -> AuthToken {
		Self {
			claims: TokenClaims::new(user.into()),
			header: Header::default(),
		}
	}

	pub fn with_alg(mut self, alg: Algorithm) -> Self {
		self.header.alg = alg;
		self
	}

	pub fn with_exp(mut self, exp: i64) -> Self {
		self.claims.exp = exp;
		self
	}

	pub fn with_nbf(mut self, nbf: i64) -> Self {
		self.claims.nbf = nbf;
		self
	}

	pub fn with_iss(mut self, iss: &[JwtIssuer]) -> Self {
		self.claims.iss.extend(iss.iter().cloned());
		self
	}

	pub fn with_aud(mut self, aud: &[JwtAudience]) -> Self {
		self.claims.aud.extend(aud.iter().cloned());
		self
	}

	pub fn encode(self, encoding_key: &EncodingKey) -> EncodeResult<String> {
		let header = &self.header;
		let claims = &self.claims;
		jsonwebtoken::encode(header, claims, encoding_key)
	}
}
