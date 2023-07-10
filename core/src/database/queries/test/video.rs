use std::str::FromStr;

use sqlx::PgPool;

use crate::database::{
    models::video_storage::StorageVideoStage,
    queries::video::{
        create, find_by_id, find_by_id_with_storage, find_by_transcription_id, CreateVideoDto,
    },
};

#[sqlx::test(migrations = "../migrations", fixtures("user", "channel"))]
async fn test_create_video(pool: PgPool) {
    let id = uuid::Uuid::new_v4();

    let dto = CreateVideoDto {
        id: &id,
        title: "Test",
        description: "Test",
        user_id: 666,
        channel_id: 666,
        language: "en",
    };

    create(&pool, dto).await.unwrap();

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

    let dto = CreateVideoDto {
        id: &id,
        title: "Test",
        description: "Test",
        user_id: 666,
        channel_id: 666,
        language: "en",
    };

    let result = create(&pool, dto).await;

    assert!(result.is_err());
}
#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
    let find_success = find_by_id(&pool, &id).await.unwrap();

    assert_eq!(find_success.id, id);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_not_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("4fa91b48-f370-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
    let find_not_success = find_by_id(&pool, &id).await;

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

    let find_success = find_by_transcription_id(&pool, transcription_id)
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
    let find_not_success = find_by_transcription_id(&pool, transcription_id).await;

    assert!(find_not_success.is_err());
}
