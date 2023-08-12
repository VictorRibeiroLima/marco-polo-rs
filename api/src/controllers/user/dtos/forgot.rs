use super::{validate_password, RE_SPECIAL_CHAR};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
pub struct Forgot {
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Reset {
    pub token: String,
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
}

#[derive(Serialize)]
pub struct ForgotPasswordEmailParams {
    pub url: String,
    pub name: String,
    pub token: String,
}
