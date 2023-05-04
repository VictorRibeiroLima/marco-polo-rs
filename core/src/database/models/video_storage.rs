use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "video_format", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoFormat {
    Mp4,
    Webm,
    Ogg,
    Mkv,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "video_stage", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoStage {
    Raw,
    Processed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideosStorage {
    pub id: i32,
    pub video_id: Uuid,
    pub storage_id: i32,
    pub stage: VideoStage,
    pub format: VideoFormat,
    pub video_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
