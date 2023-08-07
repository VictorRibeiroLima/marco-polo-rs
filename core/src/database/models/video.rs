use std::{fmt::Display, str::FromStr};

use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{channel::Channel, video_storage::VideosStorage};

//TODO: add youtube_id
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Paginate, Filtrate, FromRow)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub channel_id: i32,
    pub url: Option<String>,
    pub language: String,
    pub stage: VideoStage,
    pub error: bool,
    pub original_url: String,
    pub original_duration: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub tags: Option<String>,
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

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize, sqlx::Type)]
#[sqlx(type_name = "videos_video_stages", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoStage {
    Downloading,
    Transcribing,
    Translating,
    Subtitling,
    Uploading,
    Done,
}

impl Display for VideoStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoStage::Downloading => write!(f, "Downloading"),
            VideoStage::Transcribing => write!(f, "Transcribing"),
            VideoStage::Translating => write!(f, "Translating"),
            VideoStage::Subtitling => write!(f, "Subtitling"),
            VideoStage::Uploading => write!(f, "Uploading"),
            VideoStage::Done => write!(f, "Done"),
        }
    }
}

impl FromStr for VideoStage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Downloading" => Ok(VideoStage::Downloading),
            "Transcribing" => Ok(VideoStage::Transcribing),
            "Translating" => Ok(VideoStage::Translating),
            "Subtitling" => Ok(VideoStage::Subtitling),
            "Uploading" => Ok(VideoStage::Uploading),
            "Done" => Ok(VideoStage::Done),
            _ => Err(format!(
                "{} is not a valid video stage. expected ('Downloading', 'Transcribing', 'Translating', 'Subtitling', 'Uploading', 'Done')",
                s
            )),
        }
    }
}
