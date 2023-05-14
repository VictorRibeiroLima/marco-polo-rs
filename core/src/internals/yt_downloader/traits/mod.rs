use async_trait::async_trait;
use uuid::Uuid;

use crate::database::models::video_storage::VideoFormat;

pub struct YoutubeVideoConfig<'a> {
    pub url: &'a str,
    pub start_time: &'a Option<String>,
    pub end_time: &'a Option<String>,
    pub format: &'a Option<VideoFormat>,
}

#[async_trait]
pub trait YoutubeDownloader {
    async fn download(
        &self,
        config: YoutubeVideoConfig<'_>,
    ) -> Result<(Vec<u8>, Uuid), Box<dyn std::error::Error + Sync + Send>>;
}
