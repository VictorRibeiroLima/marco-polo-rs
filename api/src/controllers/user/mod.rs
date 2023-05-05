use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use jsonwebtoken::EncodingKey;
use marco_polo_rs_core::database::queries::{self, user::CreateUserDto};

use validator::Validate;

use crate::{controllers::user::dtos::create::CreateUser, GlobalState};
use crate::{
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
};

use self::dtos::login::Login;

mod dtos;

#[post("/")]
async fn create_user(
    global_state: web::Data<GlobalState>,
    body: Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let pool = &global_state.pool;

    let user_email = queries::user::find_by_email(pool, &body.email).await?;
    if user_email.is_some() {
        return Err(AppError::bad_request("Email already exists".into()));
    }

    let db_dto = CreateUserDto {
        name: &body.name,
        email: &body.email,
        password: &body.password,
        role: &body.role,
    };

    queries::user::create(pool, db_dto).await?;

    return Ok(HttpResponse::Created().finish());
}

#[post("/login")]
async fn login(
    global_state: web::Data<GlobalState>,
    body: Json<Login>,
) -> Result<impl Responder, AppError> {
    let pool = &global_state.pool;

    let jwt_secret =
        std::env::var("API_JSON_WEB_TOKEN_SECRET").expect("API_JSON_WEB_TOKEN_SECRET not set");
    let jwt_exp = chrono::Utc::now().timestamp() as usize + 60 * 180; // Set the token expiration to 3 hour
    let encoding_key = &EncodingKey::from_secret(jwt_secret.as_ref());

    let user = queries::user::find_by_email(pool, &body.email).await?;
    if user.is_none() {
        return Err(AppError::not_found("Invalid email or password".into()));
    }

    let user = user.unwrap();
    let dirty_password = &body.password;

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

    let response = dtos::login::LoginResponse { token };
    let response = AppResult::new(response);

    return Ok(Json(response));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/user").service(create_user).service(login);
    config.service(scope);
}
