use actix_web::{
    get,
    web::{self, post, Json},
    HttpResponse, Responder,
};

use marco_polo_rs_core::{
    database::queries::{self},
    internals::cloud::{
        aws::AwsCloudService,
        traits::{CloudService, QueueClient},
    },
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    middleware::jwt_token::TokenClaims, models::error::AppError, AppCloudService, AppPool,
};

use self::dtos::create::{CreateVideo, VideoDTO};

mod dtos;
mod service;
#[cfg(test)]
mod test;

async fn create_video<CS: CloudService>(
    pool: web::Data<AppPool>,
    cloud_service: web::Data<AppCloudService<CS>>,
    jwt: TokenClaims,
    body: Json<CreateVideo>,
) -> Result<impl Responder, AppError> {
    body.validate()?;
    let pool = &pool.pool;
    let body = body.into_inner();
    queries::channel::find_by_id(pool, body.channel_id).await?;

    let video_id = uuid::Uuid::new_v4();

    let queue_client = cloud_service.client.queue_client();
    queue_client
        .send_message(body.clone().into(video_id))
        .await?;

    service::create_video(pool, &body, jwt.id, video_id).await?;

    return Ok(HttpResponse::Created().finish());
}

#[get("/{id}")]
async fn find_by_id(
    id: web::Path<Uuid>,
    pool: web::Data<AppPool>,
    _jwt: TokenClaims,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let pool = &pool.pool;

    let user = queries::video::find_by_id(pool, &id).await?;
    let dto: VideoDTO = user.into();

    return Ok(Json(dto));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/video");
    let scope = scope
        .route("/", post().to(create_video::<AwsCloudService>))
        .service(find_by_id);
    config.service(scope);
}
