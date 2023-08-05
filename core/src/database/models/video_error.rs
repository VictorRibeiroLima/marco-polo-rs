use chrono::NaiveDateTime;
use marco_polo_rs_macros::{Filtrate, Paginate};
use sqlx::PgPool;
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

pub async fn find_video_error(pool: &PgPool, id: &Uuid) -> Result<VideoError, sqlx::Error> {
    let video_error: VideoError = sqlx::query_as!(
        VideoError,
        r#"
        SELECT id, video_id, error, created_at, stage as "stage: VideoStage"
        FROM videos_errors
        WHERE video_id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(video_error)
}
