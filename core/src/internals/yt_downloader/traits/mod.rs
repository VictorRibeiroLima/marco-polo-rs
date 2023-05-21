use async_trait::async_trait;

use crate::{
    database::models::video_storage::VideoFormat,
    internals::cloud::models::payload::VideoDownloadPayload,
};

pub struct YoutubeVideoConfig {
    pub url: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub format: Option<VideoFormat>,
}

impl From<VideoDownloadPayload> for YoutubeVideoConfig {
    fn from(payload: VideoDownloadPayload) -> Self {
        Self {
            url: payload.video_url,
            start_time: payload.start_time,
            end_time: payload.end_time,
            format: Some(payload.video_format),
        }
    }
}

#[async_trait]
pub trait YoutubeDownloader {
    async fn download(
        &self,
        config: YoutubeVideoConfig,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>>;
}
