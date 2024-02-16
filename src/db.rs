use axum::{
	async_trait,
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};
use sqlx::{pool::PoolConnection, PgPool, Postgres};

use crate::error::HandlerError;

pub struct DbConn(pub PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DbConn
where
	PgPool: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let pool = PgPool::from_ref(state);
		let conn = pool.acquire().await.map_err(HandlerError::from)?;
		Ok(Self(conn))
	}
}
