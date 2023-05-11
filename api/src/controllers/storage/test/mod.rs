use std::sync::Arc;

use super::*;
use actix_web::{http::header::ContentType, test, web, App};
use marco_polo_rs_core::internals::cloud::test::TestCloudService;

#[actix_web::test]
async fn test_create_signed_upload_url() {
    let test_cloud_service = TestCloudService::new();

    let app_cloud_service = AppCloudService {
        client: Arc::new(test_cloud_service),
    };

    let app_data = web::Data::new(app_cloud_service);

    let app = App::new().app_data(app_data).route(
        "/signed-upload-url",
        web::get().to(signed_upload_url::<TestCloudService>),
    );

    let app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/signed-upload-url")
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: AppResult<String> = test::read_body_json(resp).await;
    assert_eq!(body.data, "https://storage.googleapis.com/3600");
}
