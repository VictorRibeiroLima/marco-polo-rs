use std::collections::HashMap;

use sqlx::{postgres::PgRow, FromRow};

use crate::database::models::{
    traits::{FromRowAlias, FromRows},
    video::Video,
};

use super::OriginalVideo;

#[derive(Debug)]
pub struct OriginalVideoWithVideos {
    pub original_video: OriginalVideo,
    pub videos: Vec<Video>,
}

impl FromRows for OriginalVideoWithVideos {
    fn from_rows(rows: &Vec<PgRow>) -> Result<Vec<OriginalVideoWithVideos>, sqlx::Error> {
        let mut original_map: HashMap<i32, OriginalVideoWithVideos> = HashMap::new();

        for row in rows {
            let original_video = OriginalVideo::from_row(row)?;
            let video = Video::from_row_alias(row, "v")?;

            original_map
                .entry(original_video.id)
                .or_insert(OriginalVideoWithVideos {
                    original_video,
                    videos: Vec::new(),
                })
                .videos
                .push(video);
        }

        return Ok(original_map.into_values().collect());
    }
}
