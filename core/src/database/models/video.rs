use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::video_storage::VideosStorage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub channel_id: i32,
    pub url: Option<String>,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

pub struct VideoWithStorage {
    pub video: Video,
    pub storage: VideosStorage,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "videos_video_stages", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoStage {
    Downloading,
    Transcribing,
    Translating,
    Subtitling,
    Done,
}