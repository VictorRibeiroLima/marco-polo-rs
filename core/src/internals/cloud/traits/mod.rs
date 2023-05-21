use std::fmt::Debug;

use async_trait::async_trait;

use crate::internals::ServiceProvider;

use super::models::payload::PayloadType;

#[async_trait]
pub trait BucketClient: ServiceProvider + Sync {
    async fn create_signed_upload_url(
        &self,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    async fn create_signed_upload_url_with_uri(
        &self,
        file_uri: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    async fn create_signed_download_url(
        &self,
        file_uri: &str,
        expires_in: Option<u16>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    async fn upload_file(
        &self,
        file_uri: &str,
        file: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn upload_file_from_path(
        &self,
        file_uri: &str,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn download_file(
        &self,
        file_uri: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;

    async fn download_file_to_path(
        &self,
        file_uri: &str,
        destination_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait]
pub trait QueueClient {
    type M: QueueMessage + Debug;
    async fn receive_message(
        &self,
    ) -> Result<Option<Vec<Self::M>>, Box<dyn std::error::Error + Send + Sync>>;
    async fn send_message(
        &self,
        payload: PayloadType,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete_message(
        &self,
        message: Self::M,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn change_message_visibility(
        &self,
        message: &Self::M,
        visibility_timeout: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub trait QueueMessage {
    fn get_message(&self) -> String;
    fn get_handle(&self) -> String;
    fn to_payload(&self) -> Result<PayloadType, Box<dyn std::error::Error + Send + Sync>>;
}

pub trait CloudService: ServiceProvider {
    type BC: BucketClient;
    type QC: QueueClient;
    fn bucket_client(&self) -> &Self::BC;
    fn queue_client(&self) -> &Self::QC;
}
