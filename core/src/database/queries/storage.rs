use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_storage::{VideoFormat, VideoStage, VideosStorage};

pub struct CreateStorageDto<'a> {
    pub video_id: &'a Uuid,
    pub video_uri: &'a str,
    pub storage_id: i32,
    pub format: VideoFormat,
    pub stage: VideoStage,
}

pub async fn create(pool: &PgPool, dto: CreateStorageDto<'_>) -> Result<(), sqlx::Error> {
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
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_by_video_id_and_stage(
    pool: &PgPool,
    video_id: &Uuid,
    video_stage: VideoStage,
) -> Result<VideosStorage, sqlx::Error> {
    let result = sqlx::query_as!(
        VideosStorage,
        r#"
        SELECT 
            id,
            video_id as "video_id: Uuid",
            storage_id,
            video_path,
            format as "format: VideoFormat",
            stage as "stage: VideoStage",
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>",
            deleted_at as "deleted_at: DateTime<Utc>"
        FROM videos_storages
            WHERE video_id = $1 AND stage = $2
        "#,
        video_id,
        video_stage as VideoStage,
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}
