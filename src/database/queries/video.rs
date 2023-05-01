use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::{
    video::Video,
    video_storage::{VideoFormat, VideoStage},
};

pub struct CreateVideoDto<'a> {
    pub video_id: Uuid,
    pub video_uri: &'a str,
    pub storage_id: i32,
    pub format: VideoFormat,
    pub stage: VideoStage,
}

pub struct UpdateVideoTranscriptionDto {
    pub video_id: Uuid,
    pub storage_id: i32,
    pub path: String,
}

pub async fn create(pool: &PgPool, dto: CreateVideoDto<'_>) -> Result<(), sqlx::Error> {
    let mut trx = pool.begin().await?;
    sqlx::query!(
        r#"
        INSERT INTO videos (id)
        VALUES ($1);
        "#,
        dto.video_id,
    )
    .execute(&mut trx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO videos_storages (video_id, storage_id, video_path, format, stage)
        VALUES ($1, $2, $3, $4, $5);
        "#,
        dto.video_id,
        dto.storage_id,
        dto.video_uri,
        dto.format as VideoFormat,
        dto.stage as VideoStage,
    )
    .execute(&mut trx)
    .await?;

    trx.commit().await?;
    Ok(())
}

pub async fn find_by_transcription_id(
    pool: &PgPool,
    transcription_id: &str,
) -> Result<Video, sqlx::Error> {
    let video = sqlx::query_as!(
        Video,
        r#"
        SELECT 
            v.id as "id: Uuid", 
            v.title,
            v.description,
            v.url,
            v.language,
            v.created_at as "created_at: DateTime<Utc>",
            v.updated_at as "updated_at: DateTime<Utc>",
            v.deleted_at as "deleted_at: DateTime<Utc>",
            v.uploaded_at as "uploaded_at: DateTime<Utc>"
        FROM 
            videos v
        INNER JOIN 
            videos_transcriptions vt ON v.id = vt.video_id
        WHERE 
            vt.transcription_id = $1
    "#,
        transcription_id
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

pub async fn update_transcription(
    pool: &PgPool,
    dto: UpdateVideoTranscriptionDto,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE videos_transcriptions SET storage_id = $1, path = $2, updated_at = NOW()
            WHERE video_id = $3;
        "#,
        dto.storage_id,
        dto.path,
        dto.video_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
