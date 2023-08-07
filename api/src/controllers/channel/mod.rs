use actix_web::{
    get,
    web::{self, get, post, put, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::{
    database::{
        models::{channel::Channel, user::UserRole},
        queries::{self, channel::UpdateChannelDto, filter::Filter, pagination::Pagination},
    },
    internals::youtube_client::{
        client::YoutubeClient, traits::YoutubeClient as YoutubeClientTrait,
    },
};

mod dto;
#[cfg(test)]
mod test;

use crate::{
    controllers::channel::dto::ChannelDTO,
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
    AppPool, AppYoutubeClient,
};

async fn create_youtube_channel<YC: YoutubeClientTrait>(
    pool: web::Data<AppPool>,
    youtube_client: web::Data<AppYoutubeClient<YC>>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let client = &youtube_client.client;
    let (url, csrf_token) = client.generate_url();
    let user_id = jwt.id;

    queries::channel::create(pool, csrf_token, user_id).await?;

    let app_response = AppResult::new(url);
    return Ok(HttpResponse::Created().json(app_response));
}

async fn new_youtube_token<YC: YoutubeClientTrait>(
    pool: web::Data<AppPool>,
    youtube_client: web::Data<AppYoutubeClient<YC>>,
    jwt: TokenClaims,
    id: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let client = &youtube_client.client;
    let id = id.into_inner();

    match jwt.role {
        UserRole::Admin => queries::channel::find_by_id(&pool, id).await?,
        UserRole::User => {
            let user_id = jwt.id;
            queries::channel::find_by_and_creator(&pool, id, user_id).await?
        }
    };

    let (url, csrf_token) = client.generate_url();

    queries::channel::update_token(&pool, csrf_token, id).await?;

    let app_response = AppResult::new(url);
    return Ok(Json(app_response));
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

#[get("/")]
async fn find_all(
    pool: web::Data<AppPool>,
    pagination: web::Query<Pagination<Channel>>,
    filter: web::Query<Filter<Channel>>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let filter = filter.into_inner();
    let pagination = pagination.into_inner();

    let channels = match jwt.role {
        UserRole::Admin => queries::channel::find_all(pool, pagination, filter).await,
        UserRole::User => {
            let user_id = jwt.id;
            queries::channel::find_all_by_owner(pool, user_id, pagination).await
        }
    }?;

    let dto: Vec<ChannelDTO> = channels.into_iter().map(|c| c.into()).collect();

    return Ok(Json(dto));
}

async fn oauth_youtube_callback<YC: YoutubeClientTrait>(
    pool: web::Data<AppPool>,
    youtube_client: web::Data<AppYoutubeClient<YC>>,
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

    let channel_info_items = match info.items {
        Some(items) => items,
        None => {
            return Err(AppError::bad_request(
                "It seems that you don't have a Youtube channel. Please, create one and retry."
                    .to_string(),
            ))
        }
    };

    let snippet = match channel_info_items.get(0) {
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
        .route(
            "youtube",
            post().to(create_youtube_channel::<YoutubeClient>),
        )
        .route(
            "youtube/resign/{id}",
            put().to(new_youtube_token::<YoutubeClient>),
        )
        .route(
            "youtube/oauth/callback",
            get().to(oauth_youtube_callback::<YoutubeClient>),
        )
        .service(find_by_id)
        .service(find_all);

    config.service(channel_scope);
}
