use super::traits::FromRowAlias;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
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
    pub size: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl FromRowAlias for VideosStorage {
    fn from_row_alias(row: &sqlx::postgres::PgRow, alias: &str) -> Result<Self, sqlx::Error> {
        let alias = alias.to_owned() + ".";
        let video_storage = VideosStorage {
            id: row.try_get(format!("{}id", alias).as_str())?,
            video_id: row.try_get(format!("{}video_id", alias).as_str())?,
            storage_id: row.try_get(format!("{}storage_id", alias).as_str())?,
            stage: row.try_get(format!("{}stage", alias).as_str())?,
            format: row.try_get(format!("{}format", alias).as_str())?,
            video_path: row.try_get(format!("{}video_path", alias).as_str())?,
            size: row.try_get(format!("{}size", alias).as_str())?,
            created_at: row.try_get(format!("{}created_at", alias).as_str())?,
            updated_at: row.try_get(format!("{}updated_at", alias).as_str())?,
            deleted_at: row.try_get(format!("{}deleted_at", alias).as_str())?,
        };

        return Ok(video_storage);
    }
}
