use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};

use crate::{
	error::{HandlerError, HandlerResult},
	utils::pwd::hash_pwd,
};

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/login", post(login))
		.route("/register", post(register))
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct LoginRequest {
	username: String,
	password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
	kind: String,
	token: String,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize, ToSchema)]
pub enum LoginErrorKind {
	#[error("Invalid credentials")]
	InvalidCredentials,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body(
        content = LoginRequest,
        description = "An object containing the username and password of the user.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successful login", body = LoginResponse),
    ),
)]
pub async fn login(
	State(pool): State<PgPool>,
	Json(body): Json<LoginRequest>,
) -> HandlerResult<Json<LoginResponse>, LoginErrorKind> {
	Ok(Json(LoginResponse {
		kind: String::from("Bearer"),
		token: String::from("token"),
	}))
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterRequest {
	email: String,
	username: String,
	password: String,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize, ToSchema)]
pub enum RegisterErrorKind {
	#[error("Invalid credentials")]
	InvalidCredentials,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body(
        content = RegisterRequest,
        description = "An object containing the register payload.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Account created", body = String),
    ),
)]
pub async fn register(
	State(pool): State<PgPool>,
	Json(payload): Json<RegisterRequest>,
) -> HandlerResult<()> {
	let password_hash = hash_pwd(payload.password).await?.into_hash_string();

	sqlx::query!(
		"INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3)",
		&payload.email,
		&payload.username,
		&password_hash
	)
	.fetch_one(&pool)
	.await
	.map_err(|err| match err {
		sqlx::Error::Database(db_err) => match db_err.kind() {
			sqlx::error::ErrorKind::UniqueViolation => HandlerError::new(
				StatusCode::CONFLICT,
				"Unique violation",
				"Email or Username already exsits",
			),
			_ => HandlerError::from(db_err),
		},
		_ => HandlerError::from(err),
	})?;

	Ok(())
}
