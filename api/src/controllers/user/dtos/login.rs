use marco_polo_rs_core::database::models::user::UserRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: i32,
    pub email: String,
    pub role: UserRole,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}
