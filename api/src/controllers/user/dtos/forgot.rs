use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Forgot {
    pub email: String,
}

#[derive(Serialize)]
pub struct ForgotPasswordEmailParams {
    pub url: String,
    pub name: String,
    pub token: String,
}
