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
        youtube_client::{client::YoutubeClient, traits::YoutubeClient as YoutubeClientTrait},
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

use self::dtos::create::Create;

mod dtos;
mod service;
#[cfg(test)]
mod test;

async fn create_video<CS: CloudService, YC: YoutubeClientTrait>(
    pool: web::Data<AppPool>,
    cloud_service: web::Data<AppCloudService<CS>>,
    youtube_client: web::Data<AppYoutubeClient<YC>>,
    jwt: TokenClaims,
    body: Json<Create>,
) -> Result<impl Responder, AppError> {
    body.validate()?;
    let pool = pool.pool.as_ref();
    let body = body.into_inner();
    let youtube_client = youtube_client.client.as_ref();
    let queue_client = cloud_service.client.queue_client();

    let ids = service::create_video(pool, body, queue_client, youtube_client, jwt).await?;

    let videos = queries::video::with_original::find_all_with_original_by_ids(pool, ids).await?;

    let dto: Vec<VideoDTO> = videos.into_iter().map(|c| c.into()).collect();

    return Ok(HttpResponse::Created().json(dto));
}

#[get("/{id}")]
async fn find_by_id(
    id: web::Path<Uuid>,
    pool: web::Data<AppPool>,
    jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let pool = &pool.pool;

    let video = match jwt.role {
        UserRole::Admin => queries::video::with_original::find_with_original(pool, &id).await?,
        UserRole::User => {
            let user_id = jwt.id;
            queries::video::with_original::find_by_user_id_with_original(pool, &id, user_id).await?
        }
    };
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

    let videos = match jwt.role {
        UserRole::Admin => {
            queries::video::with_original::find_all_with_original(
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
            queries::video::with_original::find_all_with_original(
                pool,
                pagination,
                video_filter,
                original_video_filter,
            )
            .await
        }
    }?;

    let dto: Vec<VideoDTO> = videos.into_iter().map(|c| c.into()).collect();

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
