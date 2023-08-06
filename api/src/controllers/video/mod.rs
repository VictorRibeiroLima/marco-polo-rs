use std::intrinsics::mir::Return;

use actix_web::{
    get,
    web::{self, post, Json},
    HttpResponse, Responder,
};

use dotenv::Error;
use marco_polo_rs_core::{
    database::{
        models::{
            user::UserRole,
            video::Video,
            video_error::{self, find_by_video_id},
        },
        queries::{self, filter::Filter, pagination::Pagination},
    },
    internals::{
        cloud::{
            aws::AwsCloudService,
            traits::{CloudService, QueueClient},
        },
        youtube_client::{self, client::YoutubeClient},
    },
};

use uuid::Uuid;
use validator::Validate;

use crate::{
    controllers::video::dtos::{ErrorDTO, VideoDTO},
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
    queue_client
        .send_message(body.clone().into(video_id))
        .await?;

    service::create_video(pool, &body, jwt.id, video_id).await?;

    let video = queries::video::find_by_id(pool, &video_id).await?;
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

    let video = queries::video::find_by_id(pool, &id).await?;
    let dto: VideoDTO = video.into();

    return Ok(Json(dto));
}

#[get("/")]
async fn find_all(
    pool: web::Data<AppPool>,
    pagination: web::Query<Pagination<Video>>,
    filter: web::Query<Filter<Video>>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pagination = pagination.into_inner();
    let filter = filter.into_inner();
    let pool = &pool.pool;

    let channels = match jwt.role {
        UserRole::Admin => queries::video::find_all(pool, pagination, filter).await,
        UserRole::User => {
            let user_id = jwt.id;
            queries::video::find_all_by_owner(pool, user_id, pagination, filter).await
        }
    }?;

    let dto: Vec<VideoDTO> = channels.into_iter().map(|c| c.into()).collect();

    return Ok(Json(dto));
}

#[get("/videos/{id}/errors")]
async fn find_video_error(
    id: web::Path<Uuid>,
    pool: web::Data<AppPool>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let pool = &pool.pool;
    let video_errors = find_by_video_id(pool, &id).await?;

    let dto: Vec<ErrorDTO> = video_errors.into_iter().map(|c| c.into()).collect();

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
        .service(find_all);
    config.service(scope);
}
