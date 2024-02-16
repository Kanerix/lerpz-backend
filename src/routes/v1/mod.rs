pub mod auth;

use auth as Auth;
use axum::Router;
use sqlx::PgPool;
use utoipa::OpenApi;

use crate::error::HandlerError;

pub fn routes() -> Router<PgPool> {
	Router::new().nest("/auth", Auth::routes())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        Auth::login,
        Auth::register,
    ),
    components(schemas(
        Auth::LoginRequest,
        Auth::LoginResponse,
        Auth::RegisterRequest,
    )),
    tags(
        (name = "Auth", description = "The authentication API"),
    )
)]
pub struct ApiDoc;
