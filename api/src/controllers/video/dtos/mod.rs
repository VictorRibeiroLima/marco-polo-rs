pub mod create;
use chrono::NaiveDateTime;
use marco_polo_rs_core::database::models::{
    video::{Video, VideoStage},
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
    pub channel_id: i32,
    pub url: Option<String>,
    pub language: String,
    pub stage: VideoStage,
    pub error: bool,
    pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub uploaded_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct ErrorDTO {
    pub id: i32,
    pub video_id: Uuid,
    pub error: String,
    pub created_at: NaiveDateTime,
    pub stage: VideoStage,
}

impl From<VideoError> for ErrorDTO {
    fn from(value: VideoError) -> Self {
        ErrorDTO {
            id: value.id,
            video_id: value.video_id,
            error: value.error,
            created_at: value.created_at,
            stage: value.stage,
        }
    }
}

impl From<Video> for VideoDTO {
    fn from(value: Video) -> Self {
        let tags = match value.tags {
            Some(tags) => Some(tags.split(";").map(|s| s.to_string()).collect()),
            None => None,
        };
        return Self {
            id: value.id,
            title: value.title,
            description: value.description,
            user_id: value.user_id,
            channel_id: value.channel_id,
            url: value.url,
            language: value.language,
            created_at: value.created_at,
            updated_at: value.updated_at,
            uploaded_at: value.uploaded_at,
            stage: value.stage,
            tags,
            error: value.error,
        };
    }
}
