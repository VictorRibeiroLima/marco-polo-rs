use crate::database::models::video::Video;

use super::OriginalVideo;

pub struct OriginalVideoWithVideos {
    pub original_video: OriginalVideo,
    pub videos: Vec<Video>,
}
