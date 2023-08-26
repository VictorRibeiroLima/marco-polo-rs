use std::sync::Arc;

use crate::controllers::test::{create_test_app, mock::cloud_service::CloudServiceMock};

use super::*;
use actix_web::{http::header::ContentType, test, web};

#[actix_web::test]
async fn test_create_signed_upload_url() {
    let test_cloud_service = CloudServiceMock::new();

    let app_cloud_service = AppCloudService {
        client: Arc::new(test_cloud_service),
    };

    let app_data = web::Data::new(app_cloud_service);

    let app = create_test_app();
    let scope = create_scope::<CloudServiceMock>();

    let app = app.app_data(app_data).service(scope);

    let app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/storage/signed-upload-url")
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: AppResult<String> = test::read_body_json(resp).await;
    assert_eq!(body.data, "https://storage.googleapis.com/3600");
}
