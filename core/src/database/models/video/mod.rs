use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use self::stage::VideoStage;

pub mod stage;

pub mod with;

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
    pub original_video_id: i32,
    pub start_time: String,
    pub end_time: Option<String>,
    pub tags: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub uploaded_at: Option<NaiveDateTime>,
}
