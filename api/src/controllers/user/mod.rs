use actix_web::{
    get, post, put,
    web::{self, post, Json},
    HttpResponse, Responder, Scope,
};

use marco_polo_rs_core::{
    database::{
        models::user::User,
        queries::{self, filter::Filter, pagination::Pagination, user::CreateUserDto},
    },
    util::security,
};

use validator::Validate;

use self::dtos::{
    forgot::{ForgotPasswordDto, ForgotPasswordEmailParams, ResetPasswordDto},
    login::Login,
};
use crate::{
    controllers::user::dtos::{create::CreateUser, find::UserDTO},
    mail::{
        engine::{handlebars::HandleBarsEngine, MailEngine},
        sender::{lettre::LettreMailer, MailSender},
    },
    AppMailer, AppPool,
};
use crate::{
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
};

mod dtos;
#[cfg(test)]
mod test;

#[post("")]
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

async fn forgot_password<E, S>(
    pool: web::Data<AppPool>,
    mailer: web::Data<AppMailer<E, S>>,
    body: Json<ForgotPasswordDto>,
) -> Result<impl Responder, AppError>
where
    E: crate::mail::engine::MailEngine,
    S: crate::mail::sender::MailSender,
{
    let pool = &pool.pool;
    let mailer = &mailer.mailer;

    let user = queries::user::find_by_email(pool, &body.email).await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError::not_found("User not found".into()));
        }
    };

    let token = crate::auth::gen_forgot_token();

    let hashed_token = security::hash::hash(&token);

    queries::user::update_forgot_token(pool, user.id, Some(&hashed_token)).await?;

    let params = ForgotPasswordEmailParams {
        url: "http://localhost:8080".into(),
        name: user.name,
        token,
    };

    mailer
        .send(
            body.email.to_string(),
            "Forgot Password".into(),
            "forgot-password",
            Some(params),
        )
        .await?;

    return Ok(HttpResponse::Ok().finish());
}

#[put("/reset-password")]
async fn reset_password(
    pool: web::Data<AppPool>,
    body: Json<ResetPasswordDto>,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let body = body.into_inner();
    body.validate()?;

    let forgot_token = &body.token;
    let password = &body.password;

    let forgot_token = security::hash::hash(forgot_token);

    let user = queries::user::find_by_forgot_token(pool, &forgot_token).await?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err(AppError::not_found("User not found".into()));
        }
    };

    queries::user::update_password(pool, user.id, password).await?;

    return Ok(HttpResponse::Ok().finish());
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

#[get("")]
async fn find_all(
    pool: web::Data<AppPool>,
    pagination: web::Query<Pagination<User>>,
    filter: web::Query<Filter<User>>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pagination = pagination.into_inner();
    let filter = filter.into_inner();
    let pool = &pool.pool;
    let users = queries::user::find_all(pool, pagination, filter).await?;

    let dtos: Vec<UserDTO> = users.into_iter().map(|user| user.into()).collect();

    return Ok(Json(dtos));
}

fn create_scope<ME: MailEngine + 'static, MS: MailSender + 'static>() -> Scope {
    let scope = web::scope("/user")
        .route("/forgot-password", post().to(forgot_password::<ME, MS>))
        .service(create_user)
        .service(login)
        .service(find_by_id)
        .service(find_all)
        .service(reset_password);

    return scope;
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = create_scope::<HandleBarsEngine, LettreMailer>();

    config.service(scope);
}
