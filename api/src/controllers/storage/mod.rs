use actix_web::{
    web::{self, Data, Json},
    Responder,
};
use marco_polo_rs_core::internals::cloud::{
    aws::AwsCloudService,
    traits::{BucketClient, CloudService},
};

use crate::{
    models::{error::AppError, result::AppResult},
    AppCloudService,
};

#[cfg(test)]
mod test;

async fn signed_upload_url<CS>(
    cloud_service: Data<AppCloudService<CS>>,
) -> Result<impl Responder, AppError>
where
    CS: CloudService,
{
    let storage_client = cloud_service.client.bucket_client();

    let result = storage_client.create_signed_upload_url(3600).await?;

    return Ok(Json(AppResult::new(result)));
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    let scope = web::scope("/storage").route(
        "/signed-upload-url",
        web::get().to(signed_upload_url::<AwsCloudService>),
    );

    config.service(scope);
}
