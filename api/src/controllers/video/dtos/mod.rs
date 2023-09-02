pub mod create;
use chrono::NaiveDateTime;
use marco_polo_rs_core::database::models::{
    video::{stage::VideoStage, with::VideoWithOriginalAndVideoChannels},
    video_error::VideoError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct VideoDTO {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub channel_ids: Vec<i32>,
    pub urls: Vec<String>,
    pub language: String,
    pub stage: VideoStage,
    pub error: bool,
    pub original_duration: Option<String>,
    pub original_url: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub uploaded_at: Option<NaiveDateTime>,
}

impl From<VideoWithOriginalAndVideoChannels> for VideoDTO {
    fn from(value: VideoWithOriginalAndVideoChannels) -> Self {
        let original = value.original;
        let video = value.video;
        let video_channels = value.video_channels;

        let tags = match video.tags {
            Some(tags) => Some(tags.split(";").map(|s| s.to_string()).collect()),
            None => None,
        };

        let (urls, channel_ids) =
            video_channels
                .into_iter()
                .fold((Vec::new(), Vec::new()), |mut acc, vc| {
                    match vc.url {
                        Some(url) => acc.0.push(url),
                        None => {}
                    };

                    acc.1.push(vc.channel_id);
                    acc
                });

        return Self {
            id: video.id,
            title: video.title,
            description: video.description,
            user_id: video.user_id,
            channel_ids,
            urls,
            language: video.language,
            created_at: video.created_at,
            updated_at: video.updated_at,
            uploaded_at: video.uploaded_at,
            stage: video.stage,
            original_duration: original.duration,
            start_time: video.start_time,
            end_time: video.end_time,
            original_url: original.url,
            tags,
            error: video.error,
        };
    }
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct VideoErrorDTO {
    pub id: i32,
    pub video_id: Uuid,
    pub error: String,
    pub created_at: NaiveDateTime,
    pub stage: VideoStage,
}

impl From<VideoError> for VideoErrorDTO {
    fn from(value: VideoError) -> Self {
        VideoErrorDTO {
            id: value.id,
            video_id: value.video_id,
            error: value.error,
            created_at: value.created_at,
            stage: value.stage,
        }
    }
}
