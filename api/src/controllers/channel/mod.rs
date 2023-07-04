use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::database::queries::{self, channel::UpdateChannelDto};

mod dto;
#[cfg(test)]
mod test;

use crate::{
    controllers::channel::dto::ChannelDTO,
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
    AppPool, AppYoutubeClient,
};

#[post("youtube")]
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

#[get("/{id}")]
async fn find_by_id(
    id: web::Path<i32>,
    pool: web::Data<AppPool>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let pool = &pool.pool;

    let channel = queries::channel::find_by_id(pool, id).await?;
    let dto: ChannelDTO = channel.into();

    return Ok(Json(dto));
}

#[get("youtube/oauth/callback")]
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

    let channel = queries::channel::find_by_csrf_token(pool, state).await?;

    let refresh_token = client.get_refresh_token(code).await?;
    let info = client.get_channel_info(refresh_token.clone()).await?;

    let snippet = match info.items.get(0) {
        Some(item) => &item.snippet,
        None => return Err(AppError::internal_server_error()),
    };

    queries::channel::update(
        pool,
        UpdateChannelDto {
            id: channel.id,
            name: snippet.title.to_string(),
            refresh_token,
        },
    )
    .await?;

    return Ok(HttpResponse::Ok().finish());
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let channel_scope = web::scope("/channel")
        .service(create_youtube_channel)
        .service(oauth_youtube_callback)
        .service(find_by_id);
    config.service(channel_scope);
}
