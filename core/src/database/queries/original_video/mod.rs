use sqlx::{PgExecutor, PgPool};

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

//TODO: tests
pub async fn count_finished_cuts(pool: &PgPool, id: i32) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as count
        FROM  
            original_videos ov
        INNER JOIN 
            videos v 
        ON 
            v.original_video_id = ov.id
        WHERE 
            ov.id = $1 
            AND 
            (v.stage != 'DOWNLOADING' OR v.stage != 'CUTTING' OR v.error = true)
    "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count.unwrap_or(0))
}

//TODO: tests
pub async fn count_videos(pool: &PgPool, id: i32) -> Result<i64, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as count
        FROM  
            original_videos ov
        INNER JOIN 
            videos v 
        ON 
            v.original_video_id = ov.id
        WHERE 
            ov.id = $1 
    "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.count.unwrap_or(0))
}
