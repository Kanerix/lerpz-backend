use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct JwtKeys {
	pub decoding: DecodingKey,
	pub encoding: EncodingKey,
}

impl From<&str> for JwtKeys {
	fn from(key: &str) -> Self {
		let decoding = key.as_bytes();
		let encoding = key.as_bytes();

		Self {
			decoding: DecodingKey::from_secret(decoding),
			encoding: EncodingKey::from_secret(encoding),
		}
	}
}

impl From<String> for JwtKeys {
	fn from(key: String) -> Self {
		let encoding = key.as_bytes();
		let decoding = key.as_bytes();

		Self {
			encoding: EncodingKey::from_secret(encoding),
			decoding: DecodingKey::from_secret(decoding),
		}
	}
}

impl JwtKeys {
	pub fn from_ed_pem(public_key: impl Into<String>, private_key: impl Into<String>) -> Self {
		let decoding = public_key.into();
		let encoding = private_key.into();

		Self {
			decoding: DecodingKey::from_ed_pem(decoding.as_bytes()).unwrap(),
			encoding: EncodingKey::from_ed_pem(encoding.as_bytes()).unwrap(),
		}
	}
}
