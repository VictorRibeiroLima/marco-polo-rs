use std::str::FromStr;

use sqlx::PgPool;
use uuid::Uuid;

use crate::database::{models::video_channel::VideoChannel, queries::video_channel::set_url};

#[sqlx::test(migrations = "../migrations", fixtures("video"))]
async fn test_set_url(pool: PgPool) {
    let url = "https://www.test.com";
    let video_id = Uuid::from_str("806b5a48-f221-11ed-a05b-0242ac120096").unwrap();
    let channel_id = 666;

    let now = chrono::Utc::now();
    let now = now.naive_local();

    set_url(&pool, video_id, channel_id, url).await.unwrap();

    let video_channel = sqlx::query_as!(
        VideoChannel,
        r#"
        SELECT * FROM videos_channels
        WHERE video_id = $1 AND channel_id = $2
        "#,
        video_id,
        channel_id,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(video_channel.url.unwrap(), url);
    assert_eq!(video_channel.error, false);
    assert_eq!(video_channel.uploaded_at.unwrap().date(), now.date());
    assert_eq!(video_channel.updated_at.date(), now.date());
}
