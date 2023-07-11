use sqlx::PgPool;
use std::str::FromStr;

use crate::database::queries::subtitling::find_by_video_id;

#[sqlx::test(migrations = "../migrations", fixtures("videos_subtitlings"))]
async fn test_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
    let find_success = find_by_video_id(&pool, &id).await.unwrap();

    assert_eq!(find_success.video_id, id);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos_subtitlings"))]
async fn test_not_find_by_video_id(pool: PgPool) {
    let id = uuid::Uuid::from_str("805b57d2-f221-11ed-a05b-0242ac120003").unwrap(); //Invalid Uuid for the test
    let find_not_success = find_by_video_id(&pool, &id).await;

    assert!(find_not_success.is_err());
}
