use lazy_static::lazy_static;
use marco_polo_rs_core::database::models::user::UserRole;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
}

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct CreateUser {
    pub name: String,
    #[validate(email(message = "Invalid Email"))]
    pub email: String,
    #[validate(
        custom(
            function = "validate_password",
            message = "Must Contain At Least One Upper Case, Lower Case and Number. Dont use spaces."
        ),
        regex(
            path = "RE_SPECIAL_CHAR",
            message = "Must Contain At Least One Special Character"
        )
    )]
    pub password: String,
    pub role: Option<UserRole>,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in password.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }
    if !has_whitespace && has_upper && has_lower && has_digit && password.len() >= 8 {
        Ok(())
    } else {
        return Err(ValidationError::new("Password Validation Failed"));
    }
}
