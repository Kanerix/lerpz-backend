use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};

use crate::error::{HandlerError, HandlerResult};

pub fn routes() -> Router<PgPool> {
	Router::new().route("/login", post(login))
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
	State(conn): State<PgPool>,
	Json(body): Json<LoginRequest>,
) -> HandlerResult<Json<LoginResponse>, LoginErrorKind> {
	"abs".parse::<i32>()?;
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
        (status = 200, description = "Successful register", body = String),
    ),
)]
pub async fn register(
	State(pool): State<PgPool>,
	Json(payload): Json<RegisterRequest>,
) -> HandlerResult<String, RegisterErrorKind> {
	let password_hash = payload.password;

	let user = match sqlx::query!(
		"INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3)",
		&payload.email,
		&payload.username,
		&password_hash
	)
	.execute(&pool)
	.await
	{
		Err(err) => {
			return match err {
				sqlx::Error::Database(db_err) => {
					return match db_err.kind() {
						sqlx::error::ErrorKind::UniqueViolation => Err(HandlerError::new(
							StatusCode::BAD_REQUEST,
							"Unique violation",
							"Email or Username already exsits",
						)),
						sqlx::error::ErrorKind::ForeignKeyViolation => Err(HandlerError::new(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Foreign key violation",
							"Something wen't wrong",
						)),
						sqlx::error::ErrorKind::CheckViolation => Err(HandlerError::new(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Check violation",
							"Something wen't wrong",
						)),
						_ => Err(HandlerError::from(db_err)),
					}
				}
				_ => Err(HandlerError::from(err)),
			}
		}
		Ok(user) => user,
	};

	Ok("8skv8".to_owned())
}
