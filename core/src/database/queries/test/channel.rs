use sqlx::PgPool;

use crate::database::{
    models::channel::ChannelOrderFields,
    queries::{
        channel::{
            change_error_state, create, find_all, find_all_by_owner, find_by_and_creator,
            find_by_id,
        },
        pagination::Pagination,
    },
};

use super::macros::test_find_all;

const CSRF_TOKEN: &str = "123has_iuf12134";

test_find_all!(Channel, ChannelOrderFields, find_all, "channels");

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

#[sqlx::test(migrations = "../migrations", fixtures("channels"))]
async fn test_find_all_by_owner(pool: PgPool) {
    let owner_id = 1;
    let pagination = Pagination::default();

    let channels = find_all_by_owner(&pool, owner_id, pagination)
        .await
        .unwrap();

    assert_eq!(channels.len(), 10);
    for channel in channels {
        assert_eq!(channel.creator_id, owner_id);
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("channels"))]
async fn test_find_all_by_owner_owner_not_found(pool: PgPool) {
    let owner_id = 0;
    let pagination = Pagination::default();

    let channels = find_all_by_owner(&pool, owner_id, pagination)
        .await
        .unwrap();

    assert_eq!(channels.len(), 0);
}

#[sqlx::test(migrations = "../migrations", fixtures("channels"))]
async fn test_find_by_id_and_creator_id(pool: PgPool) {
    let channel_id = 1;
    let creator_id = 1;

    let channel = find_by_and_creator(&pool, channel_id, creator_id)
        .await
        .unwrap();

    assert_eq!(channel.id, channel_id);
    assert_eq!(channel.creator_id, creator_id);
}

#[sqlx::test(migrations = "../migrations", fixtures("channels"))]
async fn test_find_by_id_and_creator_id_not_found(pool: PgPool) {
    let channel_id = 1;
    let creator_id = 2;

    let channel = find_by_and_creator(&pool, channel_id, creator_id).await;

    assert!(channel.is_err());

    match channel {
        Err(error) => match error {
            sqlx::Error::RowNotFound => {}
            _ => panic!("Unexpected error"),
        },
        _ => {}
    }
}

#[sqlx::test(migrations = "../migrations", fixtures("channel"))]
async fn test_change_error_state(pool: PgPool) {
    let channel_id = 666;

    let result = change_error_state(&pool, channel_id, true).await;

    assert!(result.is_ok());

    let record = sqlx::query!(
        r#"
            SELECT error,updated_at FROM channels WHERE id = $1
        "#,
        channel_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let error = record.error;
    let updated_at = record.updated_at;

    let today = chrono::Utc::now().date_naive();

    assert!(error);
    assert_eq!(updated_at.date(), today);
}
