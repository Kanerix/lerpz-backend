use jsonwebtoken::{errors::Result as DecodeResult, DecodingKey, TokenData, Validation};

use super::claims::{JwtAudience, JwtIssuer, TokenClaims};

pub use jsonwebtoken::Algorithm;

/// Represent a validator for a JWT token.
///
/// This struct is used to validate a JWT token.
pub struct AuthTokenValidator {
	token: String,
	validation: Validation,
}

impl AuthTokenValidator {
	/// Creates a new [`AuthTokenValidator`] with the given token.
	///
	/// The default algorithm is `EdDSA` for validation. This can be
	/// changed with the `with_alg()` method.
	pub fn new(token: impl Into<String>) -> Self {
		Self {
			token: token.into(),
			validation: Validation::new(jsonwebtoken::Algorithm::EdDSA),
		}
	}

	/// Adds a single algorithm to the validation.
	pub fn with_alg(mut self, alg: jsonwebtoken::Algorithm) -> Self {
		self.validation.algorithms = vec![alg];
		self
	}

	/// Adds multiple algorithms to the validation.
	pub fn with_multiple_algs(mut self, algs: Vec<jsonwebtoken::Algorithm>) -> Self {
		self.validation.algorithms = algs;
		self
	}

	/// Enables validation of the `nbf` claim.
	pub fn with_nbf_validation(mut self) -> Self {
		self.validation.validate_nbf = true;
		self
	}

	/// Adds given issuer to the validation of the `iss` field.
	pub fn with_iss(mut self, iss: &[JwtIssuer]) -> Self {
		self.validation.set_issuer(iss);
		self
	}

	/// Adds given audience to the validation of the `aud` field.
	pub fn with_aud(mut self, aud: &[JwtAudience]) -> Self {
		self.validation.validate_aud = true;
		self.validation.set_audience(aud);
		self
	}

	/// Decoded the token with the given decoding key.
	pub fn decode(self, decoding_key: &DecodingKey) -> DecodeResult<TokenData<TokenClaims>> {
		let token = &self.token;
		let validation = &self.validation;
		jsonwebtoken::decode(&token, decoding_key, &validation)
	}
}
