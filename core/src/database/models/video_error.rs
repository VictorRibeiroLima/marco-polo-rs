use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};

use uuid::Uuid;

use super::video::VideoStage;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow, Paginate, Filtrate)]
pub struct VideoError {
    pub id: i32,
    pub video_id: Uuid,
    pub error: String,
    pub created_at: NaiveDateTime,
    pub stage: VideoStage,
}
