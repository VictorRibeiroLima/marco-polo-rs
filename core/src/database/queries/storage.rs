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

mod test {
    use std::str::FromStr;

    use sqlx::PgPool;

    use crate::database::models::video_storage::{VideoFormat, VideoStage};

    #[sqlx::test(migrations = "../migrations", fixtures("videos", "service_providers"))]
    async fn test_create_storage_mp4_raw(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();

        let dto = super::CreateStorageDto {
            video_id: &id,
            video_uri: "https://example.com/video.mp4",
            storage_id: 1234,
            format: VideoFormat::Mp4,
            stage: VideoStage::Raw,
        };

        let result = super::create(&pool, dto).await;
        assert!(result.is_ok());

        let count = sqlx::query!(
            "SELECT COUNT(*) AS count FROM videos_storages WHERE video_id = $1",
            id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(count.count.unwrap(), 1);
    }
}
