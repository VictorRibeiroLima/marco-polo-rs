use actix_web::{post, web, Responder};

use crate::{middleware::jwt_token::TokenClaims, models::error::AppError, GlobalState};

use self::dtos::create::CreateVideo;

mod dtos;

#[post("/")]
async fn create_video(
    _global_state: web::Data<GlobalState>,
    jwt: TokenClaims,
    body: web::Json<CreateVideo>,
) -> Result<impl Responder, AppError> {
    println!("create_video: {:?}", body);
    println!("create_video: {:?}", jwt);
    Ok("create_video".to_string())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/video");
    let scope = scope.service(create_video);
    config.service(scope);
}
