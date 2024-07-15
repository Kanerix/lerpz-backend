use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::{
		pwd::{hash_pwd, validate_pwd},
		token::generate_access_token,
	},
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
	Json(payload): Json<LoginRequest>,
) -> HandlerResult<Json<LoginResponse>> {
	let user = sqlx::query_as!(
		models::user::User,
		"SELECT
        id,
        email,
        username,
        password_hash,
        role AS \"role: models::user::UserRole\",
        created_at,
        updated_at
        FROM users WHERE email = $1",
		&payload.username,
	)
	.fetch_one(&pool)
	.await
	.map_err(|err| match err {
		sqlx::Error::RowNotFound => HandlerError::new(
			StatusCode::NOT_FOUND,
			"User not found",
			format!(
				"No user with the username \"{}\" was found",
				payload.username
			),
		),
		sqlx::Error::Database(db_err) => match db_err.kind() {
			sqlx::error::ErrorKind::UniqueViolation => HandlerError::new(
				StatusCode::CONFLICT,
				"Unique violation",
				"Email or username already exsits",
			),
			_ => HandlerError::from(db_err),
		},
		_ => HandlerError::from(err),
	})?;

	// TODO: Change so passwords has its own table and then make salt a field in the password table.
	// TODO: Maybe fix clone on password_hash.
	if !validate_pwd(payload.password, user.password_hash.clone(), None).await? {
		return Err(HandlerError::unauthorized());
	}

	let token = generate_access_token(user)?;

	Ok(Json(LoginResponse {
		kind: "Bearer".to_string(),
		token,
	}))
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterRequest {
	email: String,
	username: String,
	password: String,
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
	let salt = Uuid::new_v4().to_string();
	let password = payload.password;
	let password_hash = hash_pwd(password, salt).await?;

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
