use actix_web::{
    web::{self, Data, Json},
    Responder,
};
use marco_polo_rs_core::internals::cloud::{aws::s3::S3Client, traits::BucketClient};
use state::StorageState;

use crate::models::{error::AppError, result::AppResult};

mod state;
#[cfg(test)]
mod test;

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

    let scope = web::scope("/storage").app_data(app_data).route(
        "/signed-upload-url",
        web::get().to(signed_upload_url::<S3Client>),
    );

    config.service(scope);
}