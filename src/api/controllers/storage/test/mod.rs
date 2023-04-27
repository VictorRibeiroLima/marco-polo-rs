use super::*;
use actix_web::{http::header::ContentType, test, web, App};

struct TestClient;

impl TestClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::internals::cloud::traits::BucketClient for TestClient {
    fn create_signed_download_url(
        &self,
        _file_uri: &str,
        expires_in: Option<u16>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "https://storage.googleapis.com/{}",
            expires_in.unwrap()
        ))
    }

    fn create_signed_upload_url(
        &self,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("https://storage.googleapis.com/{}", expires_in))
    }
}

#[actix_web::test]
async fn test_create_signed_upload_url() {
    let bucket_name = "test-bucket".to_string();
    let storage_client = TestClient::new();
    let storage_state = StorageState::new(bucket_name, storage_client);

    let app_data = web::Data::new(storage_state);

    let app = App::new().app_data(app_data).route(
        "/signed-upload-url",
        web::get().to(signed_upload_url::<TestClient>),
    );

    let app = test::init_service(app).await;

    let req = test::TestRequest::get()
        .uri("/signed-upload-url")
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: AppResult<String> = test::read_body_json(resp).await;
    assert_eq!(body.data, "https://storage.googleapis.com/test-bucket/3600");
}
