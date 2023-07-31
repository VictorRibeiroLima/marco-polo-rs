use google_youtube3::api::Video;
use marco_polo_rs_core::{
    database::models::video::VideoWithStorageAndChannel,
    internals::youtube_client::{channel_info::ChannelInfo, traits::YoutubeClient},
    SyncError,
};

pub const CSRF_TOKEN: &str = "111aaa11aa";
pub struct YoutubeClientMock;

#[async_trait::async_trait]
impl YoutubeClient for YoutubeClientMock {
    fn generate_url(&self) -> (String, String) {
        return (
            String::from("https://youtube.com"),
            String::from(CSRF_TOKEN),
        );
    }

    async fn get_refresh_token(&self, _code: String) -> Result<String, SyncError> {
        return Ok(String::from("refresh_token"));
    }

    async fn get_channel_info(&self, _refresh_token: String) -> Result<ChannelInfo, SyncError> {
        return Ok(ChannelInfo::default());
    }

    async fn upload_video(&self, _: &VideoWithStorageAndChannel) -> Result<Video, SyncError> {
        return Ok(Default::default());
    }
}
