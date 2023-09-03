use actix_web::{
    web::{self, get, post, put, Json},
    HttpResponse, Responder, Scope,
};
use marco_polo_rs_core::{
    database::{
        models::{
            channel::{
                auth::{data::Oath2Data, AuthType},
                platform::Platform,
            },
            user::UserRole,
        },
        queries::{
            self,
            channel::{CreateChannelDto, UpdateChannelDto},
        },
    },
    internals::video_platform::youtube::traits::YoutubeClient as YoutubeClientTrait,
};

use crate::{
    middleware::jwt_token::TokenClaims,
    models::{error::AppError, result::AppResult},
    AppPool, AppYoutubeClient,
};

use super::dto;

async fn create_youtube_channel<YC: YoutubeClientTrait>(
    pool: web::Data<AppPool>,
    youtube_client: web::Data<AppYoutubeClient<YC>>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let client = &youtube_client.client;
    let (url, csrf_token) = client.generate_url();
    let user_id = jwt.id;

    let auth = AuthType::Oauth2(Oath2Data {
        csrf_token: Some(csrf_token),
        refresh_token: None,
    });

    let dto = CreateChannelDto {
        auth,
        creator_id: user_id,
        platform: Platform::Youtube,
    };

    queries::channel::create(pool, dto).await?;

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

pub fn create_scope<YC: YoutubeClientTrait + 'static>() -> Scope {
    let create_youtube_channel = post().to(create_youtube_channel::<YC>);
    let new_youtube_token = put().to(new_youtube_token::<YC>);
    let callback = get().to(oauth_youtube_callback::<YC>);

    let youtube_scope = web::scope("/youtube")
        .route("", create_youtube_channel)
        .route("resign/{id}", new_youtube_token)
        .route("oauth/callback", callback);

    return youtube_scope;
}
