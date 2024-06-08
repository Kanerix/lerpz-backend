use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{header, request::Parts},
};

use crate::error::HandlerError;

#[derive(Debug, Clone)]
pub struct AuthUser {
	pub email: String,
	pub username: String,
	pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let header = parts
			.headers
			.get(header::AUTHORIZATION)
			.and_then(|header| header.to_str().ok());

		let header_value = match header {
			Some(header) => header,
			None => return Err(HandlerError::unauthorized()),
		};

		todo!()
	}
}
