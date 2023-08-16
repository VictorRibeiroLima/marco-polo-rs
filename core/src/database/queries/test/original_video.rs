use sqlx::PgPool;

use crate::database::{
    models::original_video::{OriginalVideo, OriginalVideoOrderFields},
    queries::{self, original_video::find_all},
};

use super::macros::test_find_all;

test_find_all!(
    OriginalVideo,
    OriginalVideoOrderFields,
    find_all,
    "original_videos"
);

#[sqlx::test(migrations = "../migrations", fixtures("original_video_with_videos"))]
async fn test_find_with_videos(pool: PgPool) {
    let original_video_id = 1;
    let original_video =
        queries::original_video::with_video::find_with_videos(&pool, original_video_id)
            .await
            .unwrap();

    assert_eq!(original_video.original_video.id, original_video_id);
    assert_eq!(original_video.videos.len(), 20);
}

#[sqlx::test(migrations = "../migrations", fixtures("original_video_with_videos"))]
async fn test_find_with_videos_not_found(pool: PgPool) {
    let original_video_id = 999;
    let original_video =
        queries::original_video::with_video::find_with_videos(&pool, original_video_id).await;

    assert!(original_video.is_err());

    let err = original_video.unwrap_err();

    match err {
        sqlx::Error::RowNotFound => {}
        _ => panic!("Expected RowNotFound error"),
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("original_video"))]
async fn test_update_duration(pool: PgPool) {
    let new_duration = "01:00:00";
    let original_video_id = 666;

    queries::original_video::update_duration(&pool, original_video_id, new_duration)
        .await
        .unwrap();

    let original_video = sqlx::query_as!(
        OriginalVideo,
        "SELECT * FROM original_videos WHERE id = $1",
        original_video_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(original_video.duration, Some(new_duration.to_string()));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create(pool: PgPool) {
    let id = queries::original_video::create(&pool, "https://example.com")
        .await
        .unwrap();

    let original_video = sqlx::query_as!(
        OriginalVideo,
        "SELECT * FROM original_videos WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(original_video.id, id);
    assert_eq!(original_video.url, "https://example.com");
}
