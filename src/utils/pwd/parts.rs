use super::LATEST_SCHEME;

/// This is the parts that every password needs to be created.
pub struct PwdParts {
	scheme_name: String,
	salt: String,
	pwd: String,
}

impl PwdParts {
	/// Creates a new [`PwdParts`] structure.
	pub fn new(pwd: String, salt: String) -> Self {
		Self {
			scheme_name: LATEST_SCHEME.into(),
			salt,
			pwd,
		}
	}

	/// Gets the name of the scheme used.
	pub fn scheme_name(&self) -> &String {
		&self.scheme_name
	}

	/// Gets the password salt used.
	pub fn salt(&self) -> &str {
		&self.salt
	}

	/// Gets the password used.
	pub fn pwd(&self) -> &str {
		&self.pwd
	}

	/// This sets the scheme used to hash the password.
	///
	/// This is unsafe to call becuase you might create an password with
	/// an old and unsafe hashing scheme. You should always use the latest if possible.
	unsafe fn with_scheme(mut self, scheme_name: String) -> Self {
		self.scheme_name = scheme_name;
		self
	}
}
