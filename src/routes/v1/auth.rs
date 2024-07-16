use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use chrono::{DateTime, Utc};
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
	#[derive(Serialize, Deserialize)]
	pub struct UserWithPassword {
		pub id: Uuid,
		pub username: String,
		pub email: String,
		pub role: models::user::UserRole,
		pub hash: String,
		pub salt: Option<String>,
		pub created_at: DateTime<Utc>,
		pub updated_at: DateTime<Utc>,
	}

	let user = sqlx::query_as!(
		UserWithPassword,
		"SELECT
        users.id,
        users.email,
        users.username,
        users.role AS \"role: models::user::UserRole\",
        users.created_at,
        users.updated_at,
        passwords.hash,
        passwords.salt
        FROM users
        INNER JOIN passwords ON users.id = passwords.user_id
        WHERE email = $1",
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
	if !validate_pwd(user.hash, payload.password, None).await? {
		return Err(HandlerError::unauthorized());
	}

	let user = models::user::User {
		id: user.id,
		username: user.username,
		email: user.email,
		role: user.role,
		created_at: user.created_at,
		updated_at: user.updated_at,
	};

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
	let hash = hash_pwd(&password, &salt).await?;

	// TODO: Transaction to make sure we only create a user of we also create password

	let user = sqlx::query_as!(
		models::user::User,
		"INSERT INTO users ( email, username )
        VALUES ($1, $2)
        RETURNING
        users.id,
        users.email,
        users.username,
        users.role AS \"role: models::user::UserRole\",
        users.created_at,
        users.updated_at",
		&payload.email,
		&payload.username,
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

	sqlx::query!(
		"INSERT INTO passwords ( hash, salt, user_id ) VALUES ($1, $2, $3)",
		&hash,
		&salt,
		&user.id,
	)
	.execute(&pool)
	.await
	.map_err(|err| HandlerError::from(err))?;

	Ok(())
}
