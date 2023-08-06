use std::str::FromStr;

use sqlx::PgPool;

use crate::database::queries;

#[sqlx::test(migrations = "../migrations", fixtures("videos_errors"))]
async fn test_find_video_error(pool: PgPool) {
    let video_id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let error = "Error message";

    let result = queries::video_error::find_by_video_id(&pool, &video_id).await;

    assert!(result.is_ok());

    let video_errors = result.unwrap();
    assert_eq!(video_errors.len(), 1);

    let video_error = &video_errors[0];
    assert_eq!(video_error.video_id, video_id);
    assert_eq!(video_error.error, error);
}

#[sqlx::test(migrations = "../migrations", fixtures("videos_errors"))]
async fn test_find_no_video_error(pool: PgPool) {
    let video_id = uuid::Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120095").unwrap();

    let result = queries::video_error::find_by_video_id(&pool, &video_id).await;

    assert!(result.is_ok());

    let video_errors = result.unwrap();
    assert_eq!(video_errors.len(), 0);
}
