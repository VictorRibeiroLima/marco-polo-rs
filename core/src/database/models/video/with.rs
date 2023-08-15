use sqlx::{postgres::PgRow, FromRow, Row};

use crate::database::models::{
    channel::Channel, original_video::OriginalVideo, video_storage::VideosStorage,
};

use super::Video;

pub struct VideoWithOriginal {
    pub video: Video,
    pub original: OriginalVideo,
}

impl FromRow<'_, PgRow> for VideoWithOriginal {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let value = VideoWithOriginal {
            video: Video {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                description: row.try_get("description")?,
                url: row.try_get("url")?,
                language: row.try_get("language")?,
                user_id: row.try_get("user_id")?,
                channel_id: row.try_get("channel_id")?,
                error: row.try_get("error")?,
                original_video_id: row.try_get("original_video_id")?,
                start_time: row.try_get("start_time")?,
                end_time: row.try_get("end_time")?,
                tags: row.try_get("tags")?,
                stage: row.try_get("stage")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
                deleted_at: row.try_get("deleted_at")?,
                uploaded_at: row.try_get("uploaded_at")?,
            },
            original: OriginalVideo {
                id: row.try_get("ov.id")?,
                url: row.try_get("ov.url")?,
                duration: row.try_get("ov.duration")?,
                created_at: row.try_get("ov.created_at")?,
                updated_at: row.try_get("ov.updated_at")?,
            },
        };

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
