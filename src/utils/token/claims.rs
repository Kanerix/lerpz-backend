use crate::models::user::{User, UserRole};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represent all the claims for the token.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenClaims {
	/// This is the ID of the token. This will be some random UUID.
	pub sub: uuid::Uuid,
	/// This is at which time the token will expire.
	pub exp: i64,
	/// This is at which time the token should be valid for.
	pub nbf: i64,
	/// This is at what time the token was issued.
	pub iat: i64,
	/// This is who issued the token.
	#[serde(skip_serializing_if = "HashSet::is_empty")]
	pub iss: HashSet<JwtIssuer>,
	/// This is what ausience the token is for.
	#[serde(skip_serializing_if = "HashSet::is_empty")]
	pub aud: HashSet<JwtAudience>,
	/// The user, that the token is for.
	pub user: TokenUser,
}

/// The user field in the claims.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenUser {
	pub id: uuid::Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
}

/// Generates the `JwtAudience` enum.
///
/// This enum is used to represent the audience of the token.
macro_rules! generate_iss {
    ($($name:ident = $val:literal),+) => {
        /// The audience of the token.
        ///
        /// This this is who/what the token is for. Most often
        /// the domain of a website or the name of an app.
        #[non_exhaustive]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
        pub enum JwtAudience {
            $(
                #[serde(rename = $val)]
                $name,
            )+
        }
    }
}

/// Generates the `JwtAudience` enum.
///
/// This enum is used to represent the audience of the token.
macro_rules! generate_aud {
    ($($name:ident = $val:literal),+)=> {
        /// The issuer of the token.
        ///
        /// This is what service created the token for the user.
        /// Whis is most often the domain of the service.
        #[non_exhaustive]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
        pub enum JwtIssuer {
            $(
                #[serde(rename = $val)]
                $name,
            )+
        }
    }
}

generate_iss! {
	MainWebsite = "lerpz.com",
	Account = "account.lerpz.com",
	Dashboard = "dashboard.lerpz.com"
}

generate_aud! {
	API = "api.lerpz.com"
}

impl TokenClaims {
	/// Create a new [`TokenClaims`] instance.
	pub fn new(user: impl Into<TokenUser>) -> Self {
		Self::from(user.into())
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
