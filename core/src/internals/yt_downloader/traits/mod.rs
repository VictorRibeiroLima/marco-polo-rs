use async_trait::async_trait;

use crate::SyncError;

#[async_trait]
pub trait YoutubeDownloader {
    async fn download(&self, url: &str) -> Result<String, SyncError>;
}
