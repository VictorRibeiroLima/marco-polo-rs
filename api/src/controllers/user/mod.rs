use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};

use marco_polo_rs_core::database::{
    models::user::User,
    queries::{self, pagination::Pagination, user::CreateUserDto},
};

use validator::Validate;

use self::dtos::login::Login;
use crate::{
    controllers::user::dtos::{create::CreateUser, find::UserDTO},
    AppPool,
};
use crate::{
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
};

mod dtos;
#[cfg(test)]
mod test;

#[post("/")]
async fn create_user(
    pool: web::Data<AppPool>,
    body: Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    body.validate()?;

    let pool = &pool.pool;

    let user_email = queries::user::find_by_email(pool, &body.email).await?;
    if user_email.is_some() {
        return Err(AppError::bad_request("Email already exists".into()));
    }

    let db_dto = CreateUserDto {
        name: &body.name,
        email: &body.email,
        password: &body.password,
        role: body.role.as_ref(),
    };

    queries::user::create(pool, db_dto).await?;

    return Ok(HttpResponse::Created().finish());
}

#[post("/login")]
async fn login(
    global_state: web::Data<AppPool>,
    body: Json<Login>,
) -> Result<impl Responder, AppError> {
    let pool = &global_state.pool;

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

    let token = crate::auth::gen_token(user).await?;

    let response = dtos::login::LoginResponse { token };
    let response = AppResult::new(response);

    return Ok(Json(response));
}

#[get("/{id}")]
async fn find_by_id(
    id: web::Path<i32>,
    pool: web::Data<AppPool>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let pool = &pool.pool;

    let user = queries::user::find_by_id(pool, id).await?;
    let dto: UserDTO = user.into();

    return Ok(Json(dto));
}

#[get("/")]
async fn find_all(
    pool: web::Data<AppPool>,
    pagination: web::Query<Pagination<User>>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pagination = pagination.into_inner();
    let pool = &pool.pool;
    let users = queries::user::find_all(pool, pagination).await?;

    let dtos: Vec<UserDTO> = users.into_iter().map(|user| user.into()).collect();

    return Ok(Json(dtos));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .service(create_user)
        .service(login)
        .service(find_by_id)
        .service(find_all);
    config.service(scope);
}
