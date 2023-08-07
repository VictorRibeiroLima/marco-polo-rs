use std::{fmt::Display, str::FromStr};

use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Admin,
    User,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "Admin"),
            UserRole::User => write!(f, "User"),
        }
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(UserRole::Admin),
            "User" => Ok(UserRole::User),
            _ => Err(format!(
                "{} is not a valid user role. expected ('Admin' or 'User')",
                s
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Filtrate, Paginate, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
