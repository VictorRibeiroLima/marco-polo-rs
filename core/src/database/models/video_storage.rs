use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "video_format", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoFormat {
    Mp4,
    Webm,
    Ogg,
    Mkv,
}

impl VideoFormat {
    pub fn to_string(&self) -> String {
        match self {
            VideoFormat::Mp4 => "mp4".to_string(),
            VideoFormat::Webm => "webm".to_string(),
            VideoFormat::Ogg => "ogg".to_string(),
            VideoFormat::Mkv => "mkv".to_string(),
        }
    }
}

impl From<String> for VideoFormat {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mp4" => VideoFormat::Mp4,
            "webm" => VideoFormat::Webm,
            "ogg" => VideoFormat::Ogg,
            "mkv" => VideoFormat::Mkv,
            _ => VideoFormat::Mp4,
        }
    }
}

impl Into<String> for VideoFormat {
    fn into(self) -> String {
        match self {
            VideoFormat::Mp4 => "mp4".to_string(),
            VideoFormat::Webm => "webm".to_string(),
            VideoFormat::Ogg => "ogg".to_string(),
            VideoFormat::Mkv => "mkv".to_string(),
        }
    }
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "video_stage", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageVideoStage {
    Raw,
    Processed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VideosStorage {
    pub id: i32,
    pub video_id: Uuid,
    pub storage_id: i32,
    pub stage: StorageVideoStage,
    pub format: VideoFormat,
    pub video_path: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
