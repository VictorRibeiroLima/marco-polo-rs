use actix_web::{
    web::{self, post},
    HttpResponse, Responder,
};
use marco_polo_rs_core::internals::cloud::{aws::s3::S3Client, traits::BucketClient};

use crate::{middleware::authorization, models::error::AppError, GlobalState};

use self::{models::WebhookRequestBody, state::AssemblyAiState};

mod models;
mod service;
mod state;

authorization!(ApiKeyMiddleware, "ASSEMBLY_AI_WEBHOOK_TOKEN");

async fn webhook<C>(
    req_body: web::Json<WebhookRequestBody>,
    global_state: web::Data<GlobalState>,
    local_state: web::Data<state::AssemblyAiState<C>>,
) -> Result<impl Responder, AppError>
where
    C: BucketClient,
{
    let pool = &global_state.pool;

    let req_body = req_body.into_inner();

    service::webhook(req_body, pool, &local_state.storage_client).await?;

    return Ok(HttpResponse::Ok());
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let storage_client = S3Client::new().unwrap();
    let storage_state = AssemblyAiState::new(storage_client);

    let app_data = web::Data::new(storage_state);

    let scope = web::scope("/assemblyai");
    let scope = scope.wrap(ApiKeyMiddleware);
    let scope = scope.app_data(app_data);
    let scope = scope.route("/transcriptions/webhook", post().to(webhook::<S3Client>));

    config.service(scope);
}
