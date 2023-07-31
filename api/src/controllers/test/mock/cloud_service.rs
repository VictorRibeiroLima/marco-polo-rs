use async_trait::async_trait;
use marco_polo_rs_core::internals::{
    cloud::{
        models::payload::{PayloadType, SrtPayload},
        traits::{BucketClient, CloudService, QueueClient, QueueMessage},
    },
    ServiceProvider,
};

pub struct ServiceProviderMock;
#[allow(dead_code)]
impl ServiceProviderMock {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServiceProvider for ServiceProviderMock {
    fn id(&self) -> i32 {
        return 1;
    }
}

#[async_trait]
impl BucketClient for ServiceProviderMock {
    async fn upload_file(
        &self,
        _file_uri: &str,
        _file: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(())
    }

    async fn upload_file_from_path(
        &self,
        _file_uri: &str,
        _file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(())
    }

    async fn create_signed_upload_url_with_uri(
        &self,
        _file_uri: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        Ok(format!("https://storage.googleapis.com/{}", expires_in))
    }

    async fn create_signed_download_url(
        &self,
        _file_uri: &str,
        expires_in: Option<u16>,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        Ok(format!(
            "https://storage.googleapis.com/{}",
            expires_in.unwrap()
        ))
    }

    async fn create_signed_upload_url(
        &self,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        Ok(format!("https://storage.googleapis.com/{}", expires_in))
    }

    async fn download_file(
        &self,
        _file_path: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Sync + Send>> {
        Ok(vec![])
    }

    async fn download_file_to_path(
        &self,
        _file_path: &str,
        _destination_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(())
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

    fn to_payload(&self) -> Result<PayloadType, Box<dyn std::error::Error + Sync + Send>> {
        Ok(PayloadType::BatukaSrtTranscriptionUpload(SrtPayload {
            srt_uri: String::from("test"),
            video_id: uuid::Uuid::new_v4(),
        }))
    }
}

#[async_trait]
impl QueueClient for ServiceProviderMock {
    type M = TestMessage;

    async fn receive_message(
        &self,
    ) -> Result<Option<Vec<Self::M>>, Box<dyn std::error::Error + Sync + Send>> {
        Ok(None)
    }

    async fn send_message(
        &self,
        _payload: PayloadType,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(())
    }

    async fn delete_message(
        &self,
        _message: Self::M,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(())
    }

    async fn change_message_visibility(
        &self,
        _message: &Self::M,
        _visibility_timeout: usize,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        Ok(())
    }
}

pub struct CloudServiceMock {
    pub bucket_client: ServiceProviderMock,
    pub queue_client: ServiceProviderMock,
}

#[allow(dead_code)]
impl CloudServiceMock {
    pub fn new() -> Self {
        Self {
            bucket_client: ServiceProviderMock::new(),
            queue_client: ServiceProviderMock::new(),
        }
    }
}

impl ServiceProvider for CloudServiceMock {
    fn id(&self) -> i32 {
        return 1;
    }
}

impl CloudService for CloudServiceMock {
    type BC = ServiceProviderMock;
    type QC = ServiceProviderMock;

    fn bucket_client(&self) -> &Self::BC {
        &self.bucket_client
    }

    fn queue_client(&self) -> &Self::QC {
        &self.queue_client
    }
}
