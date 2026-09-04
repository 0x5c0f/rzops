use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User entity — maps to the "user" table.
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub full_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
