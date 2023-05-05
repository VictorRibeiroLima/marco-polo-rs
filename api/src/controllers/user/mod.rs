use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::database::queries::{self, user::CreateUserDto};
use validator::Validate;

use crate::models::error::AppError;
use crate::{controllers::user::dtos::create::CreateUser, GlobalState};

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

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/user").service(create_user);
    config.service(scope);
}
