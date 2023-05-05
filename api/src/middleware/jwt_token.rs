use actix_web::FromRequest;
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use marco_polo_rs_core::database::models::user::UserRole;

use serde::{Deserialize, Serialize};

use crate::models::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: i32,
    pub email: String,
    pub role: UserRole,
    pub exp: usize,
}

impl FromRequest for TokenClaims {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let jwt_secret =
            std::env::var("API_JSON_WEB_TOKEN_SECRET").expect("API_JSON_WEB_TOKEN_SECRET not set");
        let key = DecodingKey::from_secret(jwt_secret.as_ref());
        let auth_header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => {
                return futures::future::ready(Err(AppError::unauthorized(
                    "Missing Authorization header".to_string(),
                )
                .into()))
            }
        };

        let token = match auth_header.to_str() {
            Ok(token) => token,
            Err(_) => {
                return futures::future::ready(Err(AppError::unauthorized(
                    "Invalid Authorization header".to_string(),
                )
                .into()))
            }
        };

        let token = token.replace("Bearer ", "");

        let claims = match decode::<TokenClaims>(&token, &key, &Validation::default()) {
            Ok(claims) => claims,
            Err(_) => {
                return futures::future::ready(Err(AppError::unauthorized(
                    "Invalid Authorization header".to_string(),
                )
                .into()))
            }
        };

        ready(Ok(claims.claims))
    }
}
