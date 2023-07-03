use chrono::{DateTime, Utc};
use marco_polo_rs_core::database::models::user::{User, UserRole};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        return Self {
            id: value.id,
            name: value.name,
            email: value.email,
            role: value.role,
            created_at: value.created_at,
            updated_at: value.updated_at,
        };
    }
}
