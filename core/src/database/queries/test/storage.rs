use std::str::FromStr;

use sqlx::PgPool;

use crate::database::{
    models::video_storage::{StorageVideoStage, VideoFormat},
    queries::storage::{create, find_by_video_id_and_stage, CreateStorageDto},
};

#[sqlx::test(migrations = "../migrations", fixtures("videos", "service_providers"))]
async fn test_create_storage(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let dto = CreateStorageDto {
        video_id: &id,
        video_uri: "www.video.com",
        storage_id: 1234,
        format: VideoFormat::Mp4,
        stage: StorageVideoStage::Raw,
        size: 1234,
    };

    let result = create(&pool, dto).await;
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

#[sqlx::test(migrations = "../migrations")]
async fn test_create_storage_if_foreign_key(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();

    let dto = CreateStorageDto {
        video_id: &id,
        video_uri: "www.video.com",
        storage_id: 1234,
        format: VideoFormat::Mp4,
        stage: StorageVideoStage::Raw,
        size: 1234,
    };

    let result = create(&pool, dto).await;
    assert!(result.is_err());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "service_providers", "video_storage")
)]
async fn test_find_by_video_id_and_stage(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let find_success = find_by_video_id_and_stage(&pool, &id, StorageVideoStage::Raw)
        .await
        .unwrap();

    assert_eq!(find_success.video_id, id);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "service_providers", "video_storage")
)]
async fn test_not_find_by_video_id_and_stage(pool: PgPool) {
    let id = uuid::Uuid::from_str("805b57d2-f221-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
    let find_error = find_by_video_id_and_stage(&pool, &id, StorageVideoStage::Raw).await;

    assert!(find_error.is_err());
}
