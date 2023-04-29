use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_transcription::VideosTranscription;

pub struct CreateTranscriptionDto {
    pub video_id: Uuid,
    pub transcription_id: String,
    pub transcriber_id: i32,
}

pub async fn find_by_video_id(
    pool: &PgPool,
    video_id: &Uuid,
) -> Result<VideosTranscription, sqlx::Error> {
    let transcription = sqlx::query_as!(
        VideosTranscription,
        r#"
        SELECT
          video_id as "video_id: Uuid",
          transcriber_id,
          transcription_id,
          storage_id,
          path,
          created_at as "created_at: DateTime<Utc>",
          updated_at as "updated_at: DateTime<Utc>",
          deleted_at as "deleted_at: DateTime<Utc>"
         FROM videos_transcriptions vt
        WHERE video_id = $1
        "#,
        video_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(transcription);
}

pub async fn create(pool: &PgPool, dto: CreateTranscriptionDto) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos_transcriptions (video_id, transcription_id, transcriber_id)
        VALUES ($1, $2, $3);
        "#,
        dto.video_id,
        dto.transcription_id,
        dto.transcriber_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
