use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub password_hash: String,
	pub role: UserRole,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum UserRole {
	ADMIN,
	MODERATOR,
	USER,
}
