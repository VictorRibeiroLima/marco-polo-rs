use crate::{
    api::models::{error::AppError, result::AppResult},
    internals::cloud::{aws::s3::S3Client, traits::BucketClient},
};

mod state;
#[cfg(test)]
mod test;

use actix_web::{
    web::{self, Data, Json},
    Responder,
};

use state::StorageState;

async fn signed_upload_url<C>(state: Data<StorageState<C>>) -> Result<impl Responder, AppError>
where
    C: BucketClient,
{
    let result = state.storage_client.create_signed_upload_url(3600)?;

    return Ok(Json(AppResult::new(result)));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let bucket_name = "batuka-static-dev/".to_string();
    let storage_client = S3Client::new().unwrap();
    let storage_state = StorageState::new(bucket_name, storage_client);

    let app_data = web::Data::new(storage_state);

    let scope = web::scope("/storage").app_data(app_data).route(
        "/signed-upload-url",
        web::get().to(signed_upload_url::<S3Client>),
    );

    config.service(scope);
}
