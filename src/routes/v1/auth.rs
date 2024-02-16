use axum::{extract::State, routing::post, Json, Router};
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
        (status = 415, description = "Unsopported meadia type", body = String),
    ),
)]
pub async fn login(
	State(conn): State<PgPool>,
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
        (status = 200, description = "Successful register", body = String),
        (status = 415, description = "Unsopported meadia type", body = String),
    ),
)]
pub async fn register(
	State(conn): State<PgPool>,
	Json(body): Json<RegisterRequest>,
) -> HandlerResult<String, RegisterErrorKind> {
	Ok(String::from("Register successful"))
}
