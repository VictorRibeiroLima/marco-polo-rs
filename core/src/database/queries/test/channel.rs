use sqlx::PgPool;

use crate::database::{
    models::channel::ChannelOrderFields,
    queries::channel::{create, find_all, find_by_id},
};

use super::macros::test_find_all;

const CSRF_TOKEN: &str = "123has_iuf12134";

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_create(pool: PgPool) {
    let fixture_user_id = 666;
    let result = create(&pool, CSRF_TOKEN.to_string(), fixture_user_id).await;

    assert!(result.is_ok());
    let record = sqlx::query!(
        r#"
            SELECT COUNT(*) FROM channels WHERE csrf_token = $1
        "#,
        CSRF_TOKEN
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(record.count.is_some());

    assert_eq!(record.count.unwrap(), 1);
}

#[sqlx::test(migrations = "../migrations", fixtures("channel"))]
async fn test_find_by_id(pool: PgPool) {
    let channel_id = 666;
    let find_success = find_by_id(&pool, channel_id).await;

    assert!(find_success.is_ok());
    assert_eq!(find_success.unwrap().id, channel_id);
}

#[sqlx::test(migrations = "../migrations", fixtures("channel"))]
async fn test_not_find_by_id(pool: PgPool) {
    let invalid_channel_id = 999;
    let find_error = find_by_id(&pool, invalid_channel_id).await;
    assert!(find_error.is_err());
}

test_find_all!(Channel, ChannelOrderFields, find_all, "channels");
