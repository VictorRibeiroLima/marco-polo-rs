use super::{validate_password, RE_SPECIAL_CHAR};
use marco_polo_rs_core::database::models::user::UserRole;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct CreateUser {
    pub name: String,
    #[validate(email(message = "Invalid Email"))]
    pub email: String,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must contain at least eight characters, including one uppercase letter, one lowercase letter, and one number. Dont use spaces."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    pub password: String,
    pub role: Option<UserRole>,
}
