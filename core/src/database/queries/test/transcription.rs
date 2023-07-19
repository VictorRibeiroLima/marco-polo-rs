use std::str::FromStr;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use sqlx::PgPool;

use crate::database::{
    models::video_transcription::VideosTranscription,
    queries::transcription::{
        create, find_by_video_id, update, CreateTranscriptionDto, UpdateVideoTranscriptionDto,
    },
};

#[sqlx::test(migrations = "../migrations", fixtures("videos"))]
async fn test_create_transcription(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();

    let dto = CreateTranscriptionDto {
        video_id: id,
        transcription_id: "Transcription_Test_Ok".to_string(),
        transcriber_id: 1,
    };

    let result = create(&pool, dto).await;
    assert!(result.is_ok());

    let count = sqlx::query!(
        "SELECT COUNT(*) FROM videos_transcriptions where video_id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(count.count.is_some());
    assert_eq!(count.count.unwrap(), 1);
}

#[sqlx::test(migrations = "../migrations")]

async fn test_create_transcription_if_foreign_key_video(pool: PgPool) {
    let id = uuid::Uuid::new_v4();

    let dto = CreateTranscriptionDto {
        video_id: id,
        transcription_id: "Transcription_Test_Err".to_string(),
        transcriber_id: 1,
    };

    let result = create(&pool, dto).await;

    assert!(result.is_err());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "videos_transcriptions")
)]
async fn test_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let find_success = find_by_video_id(&pool, &id).await.unwrap();

    assert_eq!(find_success.video_id, id);
}
#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "videos_transcriptions")
)]
async fn test_not_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("805b57d2-f221-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
    let find_not_success = find_by_video_id(&pool, &id).await;

    assert!(find_not_success.is_err());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("videos", "videos_transcriptions")
)]
async fn test_update_transcription(pool: PgPool) {
    let old_path = "/path";
    let old_storage_id = 1234;

    let id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let new_storage_id = 5678;
    let new_path = "/new/path";

    let dto = UpdateVideoTranscriptionDto {
        video_id: id,
        storage_id: new_storage_id,
        path: new_path.to_string(),
    };

    let result = update(&pool, dto).await;
    assert!(result.is_ok());

    let updated_transcription = sqlx::query_as!(
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
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let updated_path = updated_transcription.path;
    let updated_storage_id = updated_transcription.storage_id;

    assert!(updated_path.is_some());
    assert!(updated_storage_id.is_some());

    let updated_path = updated_path.unwrap();
    let updated_storage_id = updated_storage_id.unwrap();

    assert_ne!(updated_path, old_path.to_string());
    assert_ne!(updated_storage_id, old_storage_id);

    assert_eq!(updated_path, new_path.to_string());
    assert_eq!(updated_storage_id, new_storage_id);
}
