use async_trait::async_trait;
use uuid::Uuid;

use crate::database::models::video_storage::VideoFormat;

pub struct YoutubeVideoConfig {
    pub url: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub format: Option<VideoFormat>,
}

#[async_trait]
pub trait YoutubeDownloader {
    async fn download(
        &self,
        config: YoutubeVideoConfig,
    ) -> Result<(Vec<u8>, Uuid), Box<dyn std::error::Error>>;
}
