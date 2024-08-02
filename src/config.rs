use axum::http::HeaderValue;
use lazy_static::lazy_static;

use crate::utils::env::{self, get_env_parse};

lazy_static! {
	/// Global configuration for the application.
	///
	/// This is loaded from environment variables and will
	/// panic if any of the required variables are missing.
	pub static ref CONFIG: Config =
		Config::from_env().unwrap_or_else(|err| panic!("couldn't load environment: {}", err));
}

/// A macro that generates a configuration struct.
///
/// The struct will have fields for each of the variables
/// and will have a `from_env` method to load the variables
/// from environment variables.
macro_rules! generate_config {
	($($name:ident: $type:ty),+) => {
		/// Configuration for the application.
		///
		/// Stores all variables used to configure the web server.
		#[allow(non_snake_case)]
		pub struct Config {
            $(
			    pub $name: $type,
            )+
		}

		impl Config {
			/// Generates a new [`Config`] from environment variables.
			///
			/// Returns an error if any of the environment variables
			/// are missing or if parsing into its type fails.
			pub fn from_env() -> env::Result<Config> {
				Ok(Config {
                    $(
                        $name: get_env_parse(stringify!($name))?,
                    )+
				})
			}
		}
	};
}

generate_config! {
	ENV: String,
	DATABASE_URL: String,
	API_ORIGIN: HeaderValue,
	PWD_SECRET: String
}
