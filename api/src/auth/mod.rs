use jsonwebtoken::EncodingKey;
use marco_polo_rs_core::database::models::user::User;

use crate::{middleware::jwt_token::TokenClaims, models::error::AppError};

pub async fn gen_token(user: User, dirty_password: &str) -> Result<String, AppError> {
    let jwt_secret =
        std::env::var("API_JSON_WEB_TOKEN_SECRET").expect("API_JSON_WEB_TOKEN_SECRET not set");
    let encoding_key = &EncodingKey::from_secret(jwt_secret.as_ref());

    let jwt_exp = chrono::Utc::now().timestamp() as usize + 60 * 180; // Set the token expiration to 3 hour

    let is_valid_password = bcrypt::verify(dirty_password, &user.password)?;

    if !is_valid_password {
        return Err(AppError::not_found("Invalid email or password".into()));
    }

    let token_claims = TokenClaims {
        id: user.id,
        email: user.email,
        role: user.role,
        exp: jwt_exp,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &token_claims,
        encoding_key,
    )?;

    return Ok(token);
}
