use std::{ffi::OsStr, str::FromStr};

use serde_json::error;

/// A type alias for handling results from this module.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when working with environment variables.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Environment variable not found")]
	NotFound,
	#[error("Error parsing environment variable")]
	ParseError,
}

/// Get an environment variable.
///
/// Returns an error if the variable is not found.
pub fn get_env<K>(key: K) -> Result<String>
where
	K: AsRef<OsStr>,
{
	std::env::var(key).map_err(|_| Error::NotFound)
}

/// Get an environment variable and parse it into a type.
///
/// Returns an error if the variable is not found or if the parsing fails.
pub fn get_env_parse<T, K>(key: K) -> Result<T>
where
	K: AsRef<OsStr>,
	T: FromStr,
{
	let variable = get_env(key)?;
	variable.parse().map_err(|_| Error::ParseError)
}
