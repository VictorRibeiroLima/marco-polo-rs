use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::database::queries;

mod dtos;
#[cfg(test)]
mod test;

use crate::{middleware::jwt_token::TokenClaims, models::error::AppError, AppPool};

use self::dtos::CreateChannel;

#[post("/")]
async fn create_channel(
    pool: web::Data<AppPool>,
    body: Json<CreateChannel>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let body = body.into_inner();
    queries::channel::create(pool, body.name).await?;

    return Ok(HttpResponse::Created().finish());
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/channel").service(create_channel);
    config.service(scope);
}
