use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_subtitling::VideosSubtitling;

pub struct CreateTranscriptionDto {
    pub video_id: Uuid,
    pub transcription_id: String,
    pub transcriber_id: i32,
}

pub async fn find_by_video_id(
    pool: &PgPool,
    video_id: &Uuid,
) -> Result<VideosSubtitling, sqlx::Error> {
    let transcription = sqlx::query_as!(
        VideosSubtitling,
        r#"
        SELECT
          video_id as "video_id: Uuid",
          subtitler_id,
          subtitling_id,
          created_at as "created_at: DateTime<Utc>",
          updated_at as "updated_at: DateTime<Utc>",
          deleted_at as "deleted_at: DateTime<Utc>"
         FROM videos_subtitlings vs
        WHERE video_id = $1
        "#,
        video_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(transcription);
}
