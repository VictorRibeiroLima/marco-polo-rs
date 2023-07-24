use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use marco_polo_rs_core::{
    database::models::{video::Video, video_storage::VideoFormat},
    internals::cloud::models::payload::{PayloadType, VideoDownloadPayload},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

lazy_static! {
    static ref YOUTUBE_URL: Regex = Regex::new(r#"^((?:https?:)?//)?((?:www|m)\.)?((?:youtube\.com|youtu.be))(/(?:[\w\-]+\?v=|embed/|v/)?)([\w\-]+)(\S+)?$"#).unwrap();
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct CreateVideo {
    pub title: String,
    pub description: String,
    #[validate(regex(path = "YOUTUBE_URL", message = "Invalid Youtube URL"))]
    pub video_url: String,
    pub channel_id: i32,
    pub language: Option<String>,
    pub format: Option<VideoFormat>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct VideoDTO {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub channel_id: i32,
    pub url: Option<String>,
    pub language: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub uploaded_at: Option<NaiveDateTime>,
}

impl From<Video> for VideoDTO {
    fn from(value: Video) -> Self {
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
        };
    }
}

impl CreateVideo {
    pub fn into(self, uuid: Uuid) -> PayloadType {
        PayloadType::BatukaDownloadVideo(VideoDownloadPayload {
            video_url: self.video_url,
            start_time: self.start_time,
            end_time: self.end_time,
            video_format: self.format.unwrap_or(VideoFormat::Mkv),
            video_id: uuid,
        })
    }
}
