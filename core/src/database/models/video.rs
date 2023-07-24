use chrono::NaiveDateTime;
use marco_polo_rs_macros::Paginate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{channel::Channel, video_storage::VideosStorage};

//TODO: add youtube_id
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Paginate, FromRow)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub channel_id: i32,
    pub url: Option<String>,
    pub language: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub uploaded_at: Option<NaiveDateTime>,
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

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "videos_video_stages", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoStage {
    Downloading,
    Transcribing,
    Translating,
    Subtitling,
    Done,
}
