use actix_web::{
    web::{self, post, Json},
    HttpResponse, Responder,
};
use marco_polo_rs_core::{
    database::queries,
    internals::cloud::{
        aws::AwsCloudService,
        traits::{CloudService, QueueClient},
    },
};
use validator::Validate;

use crate::{
    middleware::jwt_token::TokenClaims, models::error::AppError, AppCloudService, AppPool,
};

use self::dtos::create::CreateVideo;

mod dtos;
mod service;

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

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/video");
    let scope = scope.route("/", post().to(create_video::<AwsCloudService>));
    config.service(scope);
}
