use std::fmt::Debug;

use async_trait::async_trait;

pub trait BucketClient {
    fn create_signed_upload_url(
        &self,
        bucket_name: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>>;
    fn create_signed_download_url(
        &self,
        bucket_name: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

#[async_trait]
pub trait QueueClient<T>
where
    T: QueueMessage + Debug,
{
    async fn receive_message(&self) -> Result<Option<Vec<T>>, Box<dyn std::error::Error>>;
    async fn send_message(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_message(&self, message: T) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait QueueMessage {
    fn get_message(&self) -> String;
    fn get_handle(&self) -> String;
}
