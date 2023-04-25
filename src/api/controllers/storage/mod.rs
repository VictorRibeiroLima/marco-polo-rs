use crate::{
    api::models::{error::AppError, result::AppResult},
    storage::{s3::S3Client, traits::Client},
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
    C: Client,
{
    let bucket_name = &state.bucket_name;
    let result = state
        .storage_client
        .create_signed_upload_url(bucket_name, 3600)?;

    return Ok(Json(AppResult::new(result)));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let bucket_name = "test-bucket".to_string();
    let storage_client = S3Client::new();
    let storage_state = StorageState::new(bucket_name, storage_client);

    let app_data = web::Data::new(storage_state);

    let scope = web::scope("/storage").app_data(app_data).route(
        "/signed-upload-url",
        web::get().to(signed_upload_url::<S3Client>),
    );

    config.service(scope);
}
