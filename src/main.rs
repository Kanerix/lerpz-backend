use std::{net::Ipv4Addr, time::Duration};

use axum::{
	extract::MatchedPath,
	http::{Method, Request},
	middleware::from_extractor,
	Router,
};
use config::web_config;
use middleware::auth::AuthUser;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::registry()
		.with(EnvFilter::from_default_env())
		.with(tracing_subscriber::fmt::layer())
		.init();

	let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
	let listener = tokio::net::TcpListener::bind(addr).await?;

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.acquire_timeout(Duration::from_secs(3))
		.connect(&web_config().DATABASE_URL)
		.await
		.expect("can't connect to database");

	sqlx::migrate!();

	let app = Router::new()
		.merge(
			SwaggerUi::new("/swagger-ui")
				.url("/api-docs/openapi.json", routes::v1::ApiDoc::openapi()),
		)
		.nest("/api/v1", routes::v1::routes())
		.with_state(pool)
		.layer(
			CorsLayer::new()
				.allow_origin(web_config().API_ORIGIN.clone())
				.allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT]),
		)
		.layer(
			TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
				let matched_path = request
					.extensions()
					.get::<MatchedPath>()
					.map(MatchedPath::as_str);

				info_span!(
					"http_request",
					method = ?request.method(),
					matched_path,
				)
			}),
		)
		.layer(ServiceBuilder::new().layer(from_extractor::<AuthUser>()));

	axum::serve(listener, app.into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await?;

	Ok(())
}

async fn shutdown_signal() {
	let ctrl_c = async {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {
			tracing::info!("Ctrl+C received, starting graceful shutdown");
		},
		_ = terminate => {
			tracing::info!("SIGTERM received, starting graceful shutdown");
		},
	}
}
