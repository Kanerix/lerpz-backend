use crate::models::user::{User, UserRole};
use serde::{Deserialize, Serialize};
use std::{
	collections::HashSet,
	fmt::{self, Display, Formatter},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenClaims {
	pub sub: uuid::Uuid,
	pub exp: i64,
	pub nbf: i64,
	pub iat: i64,
	#[serde(skip_serializing_if = "HashSet::is_empty")]
	pub iss: HashSet<JwtIssuer>,
	#[serde(skip_serializing_if = "HashSet::is_empty")]
	pub aud: HashSet<JwtAudience>,
	pub user: TokenUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenUser {
	pub id: uuid::Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JwtAudience {
	#[serde(rename = "https://lerpz.com")]
	MainWebsite,
	#[serde(rename = "https://account.lerpz.com")]
	Account,
	#[serde(rename = "https://dashboard.lerpz.com")]
	Dashboard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum JwtIssuer {
	#[serde(rename = "https://api.lerpz.com")]
	API,
}

impl TokenClaims {
	pub fn new(user: impl Into<TokenUser>) -> Self {
		Self {
			sub: uuid::Uuid::new_v4(),
			exp: chrono::Utc::now().timestamp() + 60 * 15,
			nbf: chrono::Utc::now().timestamp(),
			iat: chrono::Utc::now().timestamp(),
			iss: HashSet::new(),
			aud: HashSet::new(),
			user: user.into(),
		}
	}
}

impl From<User> for TokenUser {
	fn from(user: User) -> Self {
		Self {
			id: user.id,
			username: user.username,
			email: user.email,
			role: user.role,
		}
	}
}

impl From<TokenUser> for TokenClaims {
	fn from(user: TokenUser) -> Self {
		Self {
			sub: uuid::Uuid::new_v4(),
			exp: chrono::Utc::now().timestamp() + 60 * 15,
			nbf: chrono::Utc::now().timestamp(),
			iat: chrono::Utc::now().timestamp(),
			iss: HashSet::new(),
			aud: HashSet::new(),
			user,
		}
	}
}

impl Display for JwtAudience {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::MainWebsite => write!(f, "https://lerpz.com"),
			Self::Account => write!(f, "https://account.lerpz.com"),
			Self::Dashboard => write!(f, "https://dashboard.lerpz.com"),
		}
	}
}

impl Display for JwtIssuer {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::API => write!(f, "https://api.lerpz.com"),
		}
	}
}
