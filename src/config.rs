use std::sync::OnceLock;

use axum::http::HeaderValue;

use crate::utils::env::{self, get_env, get_env_parse};

/// Generates a new [`Config`] from the environment.
///
/// This stores the [`Config`] in a [`OnceLock`] so that
/// it's only generated once. This allows you to call this
/// function each time you need a variable from the [`Config`].
pub fn web_config() -> &'static Config {
	static ENVIRONMENT: OnceLock<Config> = OnceLock::new();

	ENVIRONMENT.get_or_init(|| {
		Config::from_env().unwrap_or_else(|err| panic!("couldn't load environment: {}", err))
	})
}

/// Configuration for the web server.
///
/// Stores all useful variables used to configure the web server.
#[allow(non_snake_case)]
pub struct Config {
	pub DATABASE_URL: String,
	pub API_ORIGIN: HeaderValue,
}

impl Config {
	/// Generates a new [`Config`] from environment variables.
	///
	/// Returns an error if any of the environment variables
	/// are missing or if parsing into its type fails.
	pub fn from_env() -> env::Result<Config> {
		Ok(Config {
			DATABASE_URL: get_env("DATABASE_URL")?,
			API_ORIGIN: get_env_parse("API_ORIGIN")?,
		})
	}
}
