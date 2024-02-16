use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::error::HandlerError;

#[derive(Debug, Clone)]
pub struct AuthUser {
	email: String,
	username: String,
	role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let user = AuthUser {
			email: "example@mail.com".into(),
			username: "example".into(),
			role: "admin".into(),
		};

		Ok(user)
	}
}
