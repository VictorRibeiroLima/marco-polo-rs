pub mod create;
pub mod find;
pub mod forgot;
pub mod login;

use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

lazy_static! {
    static ref RE_SPECIAL_CHAR: Regex = Regex::new("^.*?[@$!%*?&].*$").unwrap();
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
