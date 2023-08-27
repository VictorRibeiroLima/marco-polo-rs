use std::collections::HashMap;

use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;

use crate::database::models::{
    channel::Channel,
    original_video::OriginalVideo,
    traits::{FromRowAlias, FromRows},
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

pub struct VideoWithChannels {
    pub video: Video,
    pub channels: Vec<Channel>,
}

impl FromRows for VideoWithChannels {
    fn from_rows(rows: &Vec<PgRow>) -> Result<Vec<Self>, sqlx::Error> {
        let mut video_map: HashMap<Uuid, VideoWithChannels> = HashMap::new();
        for row in rows {
            let video = Video::from_row(row)?;
            let id = video.id;
            let channel = Channel::from_row_alias(row, "c")?;

            video_map
                .entry(id)
                .or_insert(VideoWithChannels {
                    video,
                    channels: Vec::new(),
                })
                .channels
                .push(channel)
        }

        return Ok(video_map.into_values().collect());
    }
}
