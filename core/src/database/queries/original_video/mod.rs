use sqlx::PgExecutor;

use crate::database::models::original_video::OriginalVideo;

use super::macros::find_all;

pub mod with_video;

find_all!(OriginalVideo, "original_videos");

pub async fn create(trx: impl PgExecutor<'_>, url: impl Into<String>) -> Result<i32, sqlx::Error> {
    let url = url.into();
    let row = sqlx::query!(
        r#"
      INSERT INTO original_videos (url)
      VALUES ($1)
      RETURNING id
    "#,
        url
    )
    .fetch_one(trx)
    .await?;

    Ok(row.id)
}

pub async fn update_duration(
    trx: impl PgExecutor<'_>,
    id: i32,
    duration: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
      UPDATE original_videos
      SET duration = $1, updated_at = NOW()
      WHERE id = $2
    "#,
        duration,
        id
    )
    .execute(trx)
    .await?;

    Ok(())
}
