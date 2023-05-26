use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_subtitling::VideosSubtitling;

pub struct CreateTranscriptionDto {
    pub video_id: Uuid,
    pub transcription_id: String,
    pub transcriber_id: i32,
}

pub async fn find_by_video_id(
    pool: &PgPool,
    video_id: &Uuid,
) -> Result<VideosSubtitling, sqlx::Error> {
    let transcription = sqlx::query_as!(
        VideosSubtitling,
        r#"
        SELECT
          video_id as "video_id: Uuid",
          subtitler_id,
          subtitling_id,
          created_at as "created_at: DateTime<Utc>",
          updated_at as "updated_at: DateTime<Utc>",
          deleted_at as "deleted_at: DateTime<Utc>"
         FROM videos_subtitlings vs
        WHERE video_id = $1
        "#,
        video_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(transcription);
}

#[cfg(test)]
mod test {

    use sqlx::PgPool;
    use std::str::FromStr;

    #[sqlx::test(migrations = "../migrations", fixtures("videos_subtitlings"))]
    async fn test_find_by_sub_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let find_success = super::find_by_video_id(&pool, &id).await.unwrap();

        assert_eq!(find_success.video_id, id);
    }

    #[sqlx::test(migrations = "../migrations", fixtures("videos_subtitlings"))]
    async fn test_not_found_by_sub_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("805b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let find_not_success = super::find_by_video_id(&pool, &id).await;

        assert!(find_not_success.is_err());
    }
}
