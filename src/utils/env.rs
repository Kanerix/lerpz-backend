use std::{
	ffi::{OsStr, OsString},
	fmt::Display,
	str::FromStr,
};

use serde_json::error;

/// A type alias for handling results from this module.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when working with environment variables.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("environment variable {0} was not found")]
	NotFound(String),
	#[error("couldn't parse environment variable {0} into {1}")]
	ParseError(String, String),
}

/// Get an environment variable.
///
/// Returns an error if the variable is not found.
pub fn get_env<K>(key: K) -> Result<String>
where
	K: AsRef<OsStr> + Copy,
{
	std::env::var(key).map_err(|_| Error::NotFound(key.as_ref().to_string_lossy().to_string()))
}

/// Get an environment variable and parse it into a type.
///
/// Returns an error if the variable is not found or if the parsing fails.
pub fn get_env_parse<T, K>(key: K) -> Result<T>
where
	K: AsRef<OsStr> + Copy,
	T: FromStr,
{
	let variable = get_env(key)?;
	variable.parse().map_err(|_| {
		Error::ParseError(
			key.as_ref().to_string_lossy().to_string(),
			std::any::type_name::<T>().to_string(),
		)
	})
}
