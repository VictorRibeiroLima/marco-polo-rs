use super::*;
use actix_web::{http::header::ContentType, test, web, App};
use async_trait::async_trait;
use marco_polo_rs_core::internals::{cloud::traits::BucketClient, ServiceProvider};

struct TestClient;

impl TestClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServiceProvider for TestClient {
    fn id() -> i32 {
        return 1;
    }
}

#[async_trait]
impl BucketClient for TestClient {
    async fn upload_file(
        &self,
        _file_path: &str,
        _file: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn create_signed_upload_url_with_uri(
        &self,
        _file_uri: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("https://storage.googleapis.com/{}", expires_in))
    }

    async fn create_signed_download_url(
        &self,
        _file_uri: &str,
        expires_in: Option<u16>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "https://storage.googleapis.com/{}",
            expires_in.unwrap()
        ))
    }

    async fn create_signed_upload_url(
        &self,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("https://storage.googleapis.com/{}", expires_in))
    }

    async fn download_file(&self, _file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}

#[actix_web::test]
async fn test_create_signed_upload_url() {
    let storage_client = TestClient::new();
    let storage_state = StorageState::new(storage_client);

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
    assert_eq!(body.data, "https://storage.googleapis.com/3600");
}
