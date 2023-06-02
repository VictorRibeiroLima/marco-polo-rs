use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder, get,
};
use marco_polo_rs_core::database::queries::{self, channel::UpdateChannelDto};

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
    return Ok(HttpResponse::Created().json(app_response));
}

#[get("/oauth/callback")]
async fn oauth_youtube_callback(
    pool: web::Data<AppPool>,
    youtube_client: web::Data<AppYoutubeClient>,
    params: web::Query<dto::OauthQueryParams>,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let client = &youtube_client.client;
    let params = params.into_inner();

    let code = params.code;
    let state = params.state;

    queries::channel::find_by_csrf_token(pool, state).await?; //check if channel exists

    let refresh_token = client.get_refresh_token(code).await?;
    client.get_channel_info(refresh_token.clone()).await?;

    queries::channel::update(
        pool,
        UpdateChannelDto {
            id: 1,
            name: "ElonMuskCortes".to_string(),
            refresh_token,
        },
    )
    .await?;

    return Ok(HttpResponse::Ok().finish());
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let youtube_scope = web::scope("/youtube")
        .service(create_youtube_channel)
        .service(oauth_youtube_callback);

    let channel_scope = web::scope("/channel").service(youtube_scope);
    config.service(channel_scope);
}
