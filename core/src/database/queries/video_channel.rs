use sqlx::PgExecutor;
use uuid::Uuid;

pub async fn set_url(
    pool: impl PgExecutor<'_>,
    video_id: Uuid,
    channel_id: i32,
    url: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos_channels
        SET url = $1, uploaded_at = NOW(), error = false, updated_at = NOW()
        WHERE video_id = $2 AND channel_id = $3
        "#,
        url,
        video_id,
        channel_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}
