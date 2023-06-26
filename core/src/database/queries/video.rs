use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::{
    video::{Video, VideoStage, VideoWithStorage, VideoWithStorageAndChannel},
    video_storage::StorageVideoStage,
};

use super::storage;

pub struct CreateVideoDto<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub channel_id: i32,
    pub language: &'a str,
}

pub async fn create(pool: &PgPool, dto: CreateVideoDto<'_>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos (id, title, description, user_id, channel_id, language)
        VALUES ($1, $2, $3, $4, $5, $6);
        "#,
        dto.id,
        dto.title,
        dto.description,
        dto.user_id,
        dto.channel_id,
        dto.language,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn change_stage(
    pool: &PgPool,
    video_id: &Uuid,
    stage: VideoStage,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET stage = $1
        WHERE id = $2
        "#,
        stage as VideoStage,
        video_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_url(pool: &PgPool, video_id: &Uuid, url: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET url = $1, stage = 'DONE', uploaded_at = NOW()
        WHERE id = $2
        "#,
        url,
        video_id,
    )
    .execute(pool)
    .await?;

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
            v.user_id,
            v.channel_id,
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

pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Video, sqlx::Error> {
    let video = sqlx::query_as!(
        Video,
        r#"
        SELECT 
            v.id as "id: Uuid", 
            v.title,
            v.description,
            v.url,
            v.language,
            v.user_id,
            v.channel_id,
            v.created_at as "created_at: DateTime<Utc>",
            v.updated_at as "updated_at: DateTime<Utc>",
            v.deleted_at as "deleted_at: DateTime<Utc>",
            v.uploaded_at as "uploaded_at: DateTime<Utc>"
        FROM 
            videos v
        WHERE 
            v.id = $1
    "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

pub async fn find_by_id_with_storage(
    pool: &PgPool,
    id: &Uuid,
    video_stage: StorageVideoStage,
) -> Result<VideoWithStorage, sqlx::Error> {
    let video = find_by_id(pool, id).await?;
    let storage = storage::find_by_video_id_and_stage(pool, id, video_stage).await?;

    Ok(VideoWithStorage { video, storage })
}

pub async fn find_by_id_with_storage_and_channel(
    pool: &PgPool,
    id: &Uuid,
    video_stage: StorageVideoStage,
) -> Result<VideoWithStorageAndChannel, sqlx::Error> {
    let video_with_storage = find_by_id_with_storage(pool, id, video_stage).await?;
    let channel =
        crate::database::queries::channel::find_by_id(pool, video_with_storage.video.channel_id)
            .await?;

    let video_with_channel = VideoWithStorageAndChannel {
        video: video_with_storage.video,
        storage: video_with_storage.storage,
        channel,
    };
    return Ok(video_with_channel);
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use sqlx::PgPool;

    use crate::database::{
        models::video_storage::StorageVideoStage, queries::video::find_by_id_with_storage,
    };

    #[sqlx::test(migrations = "../migrations", fixtures("user", "channel"))]
    async fn test_create_video(pool: PgPool) {
        let id = uuid::Uuid::new_v4();

        let dto = super::CreateVideoDto {
            id: &id,
            title: "Test",
            description: "Test",
            user_id: 666,
            channel_id: 666,
            language: "en",
        };

        super::create(&pool, dto).await.unwrap();

        let count = sqlx::query!("SELECT COUNT(*) FROM videos where id = $1", id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(count.count.is_some());
        assert_eq!(count.count.unwrap(), 1);
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create_if_foreign_key(pool: PgPool) {
        let id = uuid::Uuid::new_v4();

        let dto = super::CreateVideoDto {
            id: &id,
            title: "Test",
            description: "Test",
            user_id: 666,
            channel_id: 666,
            language: "en",
        };

        let result = super::create(&pool, dto).await;

        assert!(result.is_err());
    }
    #[sqlx::test(migrations = "../migrations", fixtures("videos"))]
    async fn test_find_by_video_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let find_success = super::find_by_id(&pool, &id).await.unwrap();

        assert_eq!(find_success.id, id);
    }

    #[sqlx::test(migrations = "../migrations", fixtures("videos"))]
    async fn test_not_find_by_video_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("4fa91b48-f370-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
        let find_not_success = super::find_by_id(&pool, &id).await;

        assert!(find_not_success.is_err());
    }

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "service_providers", "video_storage")
    )]

    async fn test_find_by_id_with_storage(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let video_stage = StorageVideoStage::Raw;
        let storage_id = 1234;

        let find_success = find_by_id_with_storage(&pool, &id, video_stage)
            .await
            .unwrap();

        assert_eq!(find_success.video.id, id);
        assert_eq!(find_success.storage.storage_id, storage_id);
    }

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "service_providers", "video_storage")
    )]

    async fn test_not_find_by_id_with_storage(pool: PgPool) {
        let id = uuid::Uuid::from_str("805b57d2-f221-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
        let video_stage = StorageVideoStage::Raw;

        let find_error = find_by_id_with_storage(&pool, &id, video_stage).await;

        assert!(find_error.is_err());
    }

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "videos_transcriptions")
    )]
    async fn test_find_by_transcription_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let transcription_id = "Transcription_Test_Ok";

        let find_success = super::find_by_transcription_id(&pool, transcription_id)
            .await
            .unwrap();

        assert_eq!(find_success.id, id);
    }

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "videos_transcriptions")
    )]
    async fn test_not_find_by_transcription_id(pool: PgPool) {
        let transcription_id = "Transcription_Test_Err";
        let find_not_success = super::find_by_transcription_id(&pool, transcription_id).await;

        assert!(find_not_success.is_err());
    }
}
