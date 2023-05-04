use crate::internals::ServiceProvider;
use async_trait::async_trait;

use super::{
    models::payload::SrtPayload,
    traits::{CloudService, QueueClient, QueueMessage},
};

pub struct TestClient;
#[allow(dead_code)]
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
impl crate::internals::cloud::traits::BucketClient for TestClient {
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

#[derive(Debug)]
pub struct TestMessage;

#[allow(dead_code)]
impl TestMessage {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl QueueMessage for TestMessage {
    fn get_message(&self) -> String {
        String::from("test")
    }

    fn get_handle(&self) -> String {
        String::from("test")
    }

    fn to_payload(
        &self,
    ) -> Result<crate::internals::cloud::models::payload::PayloadType, Box<dyn std::error::Error>>
    {
        Ok(
            crate::internals::cloud::models::payload::PayloadType::BatukaSrtTranscriptionUpload(
                SrtPayload {
                    srt_uri: String::from("test"),
                    video_id: uuid::Uuid::new_v4(),
                },
            ),
        )
    }
}

#[async_trait]
impl QueueClient for TestClient {
    type M = TestMessage;

    async fn receive_message(&self) -> Result<Option<Vec<Self::M>>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    async fn send_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn delete_message(&self, _message: Self::M) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn change_message_visibility(
        &self,
        _message: &Self::M,
        _visibility_timeout: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub struct TestCloudService {
    pub bucket_client: TestClient,
    pub queue_client: TestClient,
}

#[allow(dead_code)]
impl TestCloudService {
    pub fn new() -> Self {
        Self {
            bucket_client: TestClient::new(),
            queue_client: TestClient::new(),
        }
    }
}

impl ServiceProvider for TestCloudService {
    fn id() -> i32 {
        return 1;
    }
}

impl CloudService for TestCloudService {
    type BC = TestClient;
    type QC = TestClient;

    fn bucket_client(&self) -> &Self::BC {
        &self.bucket_client
    }

    fn queue_client(&self) -> &Self::QC {
        &self.queue_client
    }
}
