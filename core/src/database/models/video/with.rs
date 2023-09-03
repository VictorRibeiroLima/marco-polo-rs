use sqlx::{postgres::PgRow, FromRow};

use crate::database::models::{
    channel::Channel, original_video::OriginalVideo, traits::FromRowAlias,
    video_storage::VideosStorage,
};

use super::Video;

pub struct VideoWithOriginal {
    pub video: Video,
    pub original: OriginalVideo,
}

impl FromRow<'_, PgRow> for VideoWithOriginal {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let video = Video::from_row(row)?;
        let original = OriginalVideo::from_row_alias(row, "ov")?;
        let value = VideoWithOriginal { video, original };
        Ok(value)
    }
}

pub struct VideoWithStorage {
    pub video: Video,
    pub storage: VideosStorage,
}

pub struct VideoWithStorageAndChannel {
    pub video: Video,
    pub storage: VideosStorage,
    pub channel: Channel,
}
