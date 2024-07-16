use axum::http::HeaderValue;
use lazy_static::lazy_static;

use crate::utils::env::{self, get_env, get_env_parse};

lazy_static! {
	/// Global configuration for the application.
	///
	/// This is loaded from environment variables and will
	/// panic if any of the required variables are missing.
	pub static ref CONFIG: Config =
		Config::from_env().unwrap_or_else(|err| panic!("couldn't load environment: {}", err));
}

/// Configuration for the application.
///
/// Stores all variables used to configure the web server.
///
/// TODO: Mabye make macro to generate this struct from env variables
#[allow(non_snake_case)]
pub struct Config {
	pub ENV: String,
	pub DATABASE_URL: String,
	pub API_ORIGIN: HeaderValue,
	pub PWD_SECRET: String,
}

impl Config {
	/// Generates a new [`Config`] from environment variables.
	///
	/// Returns an error if any of the environment variables
	/// are missing or if parsing into its type fails.
	pub fn from_env() -> env::Result<Config> {
		Ok(Config {
			ENV: get_env("ENV")?,
			DATABASE_URL: get_env("DATABASE_URL")?,
			API_ORIGIN: get_env_parse("API_ORIGIN")?,
			PWD_SECRET: get_env("PWD_SECRET")?,
		})
	}
}
