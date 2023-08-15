use actix_web::{
    get,
    web::{self, post, Json},
    HttpResponse, Responder,
};

use marco_polo_rs_core::{
    database::{
        models::{user::UserRole, video::Video},
        queries::{self, filter::Filter, pagination::Pagination},
    },
    internals::{
        cloud::{aws::AwsCloudService, traits::CloudService},
        youtube_client::{self, client::YoutubeClient},
    },
};

use uuid::Uuid;
use validator::Validate;

use crate::{
    controllers::video::dtos::{VideoDTO, VideoErrorDTO},
    middleware::jwt_token::TokenClaims,
    models::error::AppError,
    AppCloudService, AppPool, AppYoutubeClient,
};

use self::dtos::create::CreateVideo;

mod dtos;
mod service;
#[cfg(test)]
mod test;

async fn create_video<CS: CloudService, YC: youtube_client::traits::YoutubeClient>(
    pool: web::Data<AppPool>,
    cloud_service: web::Data<AppCloudService<CS>>,
    youtube_client: web::Data<AppYoutubeClient<YC>>,
    jwt: TokenClaims,
    body: Json<CreateVideo>,
) -> Result<impl Responder, AppError> {
    body.validate()?;
    let pool = &pool.pool;
    let body = body.into_inner();
    let youtube_client = &youtube_client.client;

    let channel = match jwt.role {
        UserRole::Admin => queries::channel::find_by_id(pool, body.channel_id).await?,
        UserRole::User => {
            let user_id = jwt.id;
            queries::channel::find_by_and_creator(pool, body.channel_id, user_id).await?
        }
    };

    if channel.error {
        return Err(AppError::bad_request(
            "Channel has errors. Please contact admins".to_string(),
        ));
    };

    let refresh_token = match channel.refresh_token {
        Some(refresh_token) => refresh_token,
        None => {
            return Err(AppError::bad_request(
                "Youtube channel not linked".to_string(),
            ))
        }
    };

    let result = youtube_client.get_channel_info(refresh_token).await;

    if result.is_err() {
        queries::channel::change_error_state(pool, body.channel_id, true).await?;
        return Err(AppError::bad_request(
            "Channel has errors. Please contact admins".to_string(),
        ));
    }

    let video_id = uuid::Uuid::new_v4();

    let queue_client = cloud_service.client.queue_client();

    service::create_video(pool, &body, queue_client, jwt.id, video_id).await?;

    let video = queries::video::find_with_original(pool, &video_id).await?;
    let dto: VideoDTO = video.into();

    return Ok(HttpResponse::Created().json(dto));
}

#[get("/{id}")]
async fn find_by_id(
    id: web::Path<Uuid>,
    pool: web::Data<AppPool>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let pool = &pool.pool;

    let video = queries::video::find_with_original(pool, &id).await?;
    let dto: VideoDTO = video.into();

    return Ok(Json(dto));
}

#[get("/")]
async fn find_all(
    pool: web::Data<AppPool>,
    pagination: web::Query<Pagination<Video>>,
    video_filter: web::Query<Filter<Video>>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pagination = pagination.into_inner();
    let mut video_filter = video_filter.into_inner();

    //TODO: refactor
    let original_video_filter = Default::default();

    let pool = &pool.pool;

    let channels = match jwt.role {
        UserRole::Admin => {
            queries::video::find_all_with_original(
                pool,
                pagination,
                video_filter,
                original_video_filter,
            )
            .await
        }
        UserRole::User => {
            let user_id = jwt.id;
            video_filter.options.user_id = Some(user_id);
            queries::video::find_all_with_original(
                pool,
                pagination,
                video_filter,
                original_video_filter,
            )
            .await
        }
    }?;

    let dto: Vec<VideoDTO> = channels.into_iter().map(|c| c.into()).collect();

    return Ok(Json(dto));
}

#[get("/{id}/errors")]
async fn find_video_errors(
    id: web::Path<Uuid>,
    pool: web::Data<AppPool>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let id = id.into_inner();

    let video_errors = match jwt.role {
        UserRole::Admin => queries::video_error::find_by_video_id(pool, &id).await?,
        UserRole::User => {
            let user_id = jwt.id;
            queries::video_error::find_by_video_id_and_owner(pool, &id, user_id).await?
        }
    };

    let dto: Vec<VideoErrorDTO> = video_errors.into_iter().map(|c| c.into()).collect();

    return Ok(Json(dto));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/video");
    let scope = scope
        .route(
            "/",
            post().to(create_video::<AwsCloudService, YoutubeClient>),
        )
        .service(find_by_id)
        .service(find_all)
        .service(find_video_errors);
    config.service(scope);
}
