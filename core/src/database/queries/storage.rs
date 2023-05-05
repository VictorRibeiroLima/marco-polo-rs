use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_storage::{VideoFormat, VideoStage};

pub struct CreateStorageDto<'a> {
    pub video_id: Uuid,
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
