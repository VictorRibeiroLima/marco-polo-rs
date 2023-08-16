use std::collections::HashMap;

use sqlx::{postgres::PgRow, FromRow};

use crate::database::models::video::Video;

use super::OriginalVideo;

#[derive(Debug)]
pub struct OriginalVideoWithVideos {
    pub original_video: OriginalVideo,
    pub videos: Vec<Video>,
}

impl OriginalVideoWithVideos {
    pub fn from_rows(rows: &Vec<PgRow>) -> Result<Vec<OriginalVideoWithVideos>, sqlx::Error> {
        let mut original_map: HashMap<i32, OriginalVideoWithVideos> = HashMap::new();

        for row in rows {
            let original_video = OriginalVideo::from_row(row)?;

            let original_video = match original_map.get_mut(&original_video.id) {
                Some(original_video) => original_video,
                None => {
                    let id = original_video.id;
                    let original_video_with_videos = OriginalVideoWithVideos {
                        original_video,
                        videos: vec![],
                    };
                    original_map.insert(id, original_video_with_videos);
                    original_map.get_mut(&id).unwrap()
                }
            };

            let video = Video::from_row_alias(row, "v")?;

            original_video.videos.push(video);
        }

        return Ok(original_map
            .into_iter()
            .map(|(_, value)| value)
            .collect::<Vec<OriginalVideoWithVideos>>());
    }
}
