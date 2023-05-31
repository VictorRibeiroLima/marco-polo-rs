use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::{database::queries, internals::youtube_client};

mod dto;
#[cfg(test)]
mod test;

use crate::{
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
    AppPool, AppYoutubeClient,
};

#[post("/")]
async fn create_youtube_channel(
    pool: web::Data<AppPool>,
    youtube_client: web::Data<AppYoutubeClient>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let client = &youtube_client.client;
    let (url, csrf_token) = client.generate_url();
    queries::channel::create(pool, csrf_token).await?;

    let app_response = AppResult::new(url);
    return Ok(Json(app_response));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let youtube_scope = web::scope("/youtube").service(create_youtube_channel);
    let channel_scope = web::scope("/channel").service(youtube_scope);
    config.service(channel_scope);
}
