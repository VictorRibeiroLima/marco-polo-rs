use async_trait::async_trait;

use crate::{internals::video_platform::VideoPlatformClient, SyncError};

use super::channel_info::ChannelInfo;

#[async_trait]
pub trait YoutubeClient: VideoPlatformClient {
    fn generate_url(&self) -> (String, String);
    async fn get_refresh_token(&self, code: String) -> Result<String, SyncError>;
    async fn get_channel_info(&self, refresh_token: String) -> Result<ChannelInfo, SyncError>;
}
