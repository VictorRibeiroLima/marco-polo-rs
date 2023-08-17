use crate::database::models::video::stage::VideoStage;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_error::VideoError;

pub async fn find_by_video_id(pool: &PgPool, id: &Uuid) -> Result<Vec<VideoError>, sqlx::Error> {
    let video_errors: Vec<VideoError> = sqlx::query_as!(
        VideoError,
        r#"
      SELECT id, video_id, error, created_at, stage as "stage: VideoStage"
      FROM videos_errors
      WHERE video_id = $1
      "#,
        id
    )
    .fetch_all(pool)
    .await?;

    Ok(video_errors)
}

pub async fn find_by_video_id_and_owner(
    pool: &PgPool,
    id: &Uuid,
    user_id: i32,
) -> Result<Vec<VideoError>, sqlx::Error> {
    let video_errors: Vec<VideoError> = sqlx::query_as!(
        VideoError,
        r#"
      SELECT vr.id, vr.video_id, vr.error, vr.created_at, vr.stage as "stage: VideoStage"
      FROM videos_errors vr
      JOIN videos v ON v.id = vr.video_id
      WHERE video_id = $1 and v.user_id = $2
      "#,
        id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(video_errors)
}
