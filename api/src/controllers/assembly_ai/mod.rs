use actix_web::{
    web::{self, post},
    HttpResponse, Responder,
};
use marco_polo_rs_core::internals::cloud::{aws::AwsCloudService, traits::CloudService};

use crate::{
    middleware::api_token::authorization, models::error::AppError, AppCloudService, AppPool,
};

use self::models::WebhookRequestBody;

mod models;
mod service;

authorization!(ApiKeyMiddleware, "ASSEMBLY_AI_WEBHOOK_TOKEN");

async fn webhook<CS>(
    req_body: web::Json<WebhookRequestBody>,
    pool: web::Data<AppPool>,
    cloud_service: web::Data<AppCloudService<CS>>,
) -> Result<impl Responder, AppError>
where
    CS: CloudService,
{
    let pool = &pool.pool;

    let bucket_client = cloud_service.client.bucket_client();

    let req_body = req_body.into_inner();

    service::webhook(req_body, pool, bucket_client).await?;

    return Ok(HttpResponse::Ok());
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/assemblyai");
    let scope = scope.wrap(ApiKeyMiddleware);
    let scope = scope.route(
        "/transcriptions/webhook",
        post().to(webhook::<AwsCloudService>),
    );

    config.service(scope);
}
