use crate::{
    api::{
        middleware::authorization,
        models::{error::AppError, result::AppResult},
    },
    internals::cloud::{aws::s3::S3Client, traits::BucketClient},
};
use actix_web::{
    web::{self, Data, Json},
    Responder,
};
use state::StorageState;

mod state;
#[cfg(test)]
mod test;

authorization!(ApiKeyMiddleware, "API_KEY");

async fn signed_upload_url<C>(state: Data<StorageState<C>>) -> Result<impl Responder, AppError>
where
    C: BucketClient,
{
    let result = state.storage_client.create_signed_upload_url(3600).await?;

    return Ok(Json(AppResult::new(result)));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let storage_client = S3Client::new().unwrap();
    let storage_state = StorageState::new(storage_client);

    let app_data = web::Data::new(storage_state);

    let scope = web::scope("/storage")
        .wrap(ApiKeyMiddleware)
        .app_data(app_data)
        .route(
            "/signed-upload-url",
            web::get().to(signed_upload_url::<S3Client>),
        );

    config.service(scope);
}
