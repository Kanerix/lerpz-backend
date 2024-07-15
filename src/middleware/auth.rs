use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{header, request::Parts},
};

use crate::{
	error::HandlerError,
	utils::token::{self, claims::TokenUser, decode_access_token},
};

#[derive(Debug, Clone)]
pub struct AuthUser(pub TokenUser);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let header = parts
			.headers
			.get(header::AUTHORIZATION)
			.and_then(|header| header.to_str().ok())
			.ok_or(HandlerError::unauthorized())?;

		let token = header
			.split_whitespace()
			.last()
			.ok_or(HandlerError::unauthorized())?;

		let token_data = decode_access_token(token).map_err(|err| match err {
			token::error::Error::TokenError(err) => HandlerError::from(err),
		})?;

		Ok(AuthUser(token_data.claims.user))
	}
}
