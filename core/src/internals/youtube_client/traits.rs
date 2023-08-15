use async_trait::async_trait;
use google_youtube3::api::Video;

use crate::{database::models::video::with::VideoWithStorageAndChannel, SyncError};

use super::channel_info::ChannelInfo;

#[async_trait]
pub trait YoutubeClient {
    fn generate_url(&self) -> (String, String);
    async fn get_refresh_token(&self, code: String) -> Result<String, SyncError>;
    async fn get_channel_info(&self, refresh_token: String) -> Result<ChannelInfo, SyncError>;
    async fn upload_video(&self, video: &VideoWithStorageAndChannel) -> Result<Video, SyncError>;
}
