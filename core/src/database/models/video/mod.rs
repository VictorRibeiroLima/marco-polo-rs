use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

use self::stage::VideoStage;

use super::traits::FromRowAlias;

pub mod stage;

pub mod with;

//TODO: add youtube_id
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Paginate, Filtrate, FromRow)]
pub struct Video {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub user_id: i32,
    pub language: String,
    pub stage: VideoStage,
    pub error: bool,
    pub original_video_id: i32,
    pub start_time: String,
    pub end_time: Option<String>,
    pub tags: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub uploaded_at: Option<NaiveDateTime>,
}

impl FromRowAlias for Video {
    fn from_row_alias(row: &PgRow, alias: &str) -> Result<Self, sqlx::Error> {
        let alias = alias.to_owned() + ".";
        let video = Video {
            id: row.try_get(format!("{}id", alias).as_str())?,
            title: row.try_get(format!("{}title", alias).as_str())?,
            description: row.try_get(format!("{}description", alias).as_str())?,
            user_id: row.try_get(format!("{}user_id", alias).as_str())?,
            language: row.try_get(format!("{}language", alias).as_str())?,
            stage: row.try_get(format!("{}stage", alias).as_str())?,
            error: row.try_get(format!("{}error", alias).as_str())?,
            original_video_id: row.try_get(format!("{}original_video_id", alias).as_str())?,
            start_time: row.try_get(format!("{}start_time", alias).as_str())?,
            end_time: row.try_get(format!("{}end_time", alias).as_str())?,
            tags: row.try_get(format!("{}tags", alias).as_str())?,
            created_at: row.try_get(format!("{}created_at", alias).as_str())?,
            updated_at: row.try_get(format!("{}updated_at", alias).as_str())?,
            deleted_at: row.try_get(format!("{}deleted_at", alias).as_str())?,
            uploaded_at: row.try_get(format!("{}uploaded_at", alias).as_str())?,
        };

        return Ok(video);
    }
}
