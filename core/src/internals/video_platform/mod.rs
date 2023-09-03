use crate::{
    database::models::{channel::Channel, video::Video, video_storage::VideosStorage},
    SyncError,
};

use self::errors::HeathCheckError;

pub mod youtube;

pub mod errors;

pub struct UploadParams<'a> {
    pub video: &'a Video,
    pub storage: &'a VideosStorage,
    pub channel: &'a Channel,
}

#[async_trait::async_trait]
pub trait VideoPlatformClient {
    type VideoResult;
    async fn upload_video<'a>(
        &self,
        video: UploadParams<'a>,
    ) -> Result<Self::VideoResult, SyncError>;

    async fn check_channel_health<'a>(
        &self,
        channel: &'a Channel,
    ) -> Result<(), HeathCheckError<'a>>;
}
