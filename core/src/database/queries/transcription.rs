use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video_transcription::VideosTranscription;

pub struct CreateTranscriptionDto {
    pub video_id: Uuid,
    pub transcription_id: String,
    pub transcriber_id: i32,
}

pub async fn find_by_video_id(
    pool: &PgPool,
    video_id: &Uuid,
) -> Result<VideosTranscription, sqlx::Error> {
    let transcription = sqlx::query_as!(
        VideosTranscription,
        r#"
        SELECT
          video_id as "video_id: Uuid",
          transcriber_id,
          transcription_id,
          storage_id,
          path,
          created_at as "created_at: DateTime<Utc>",
          updated_at as "updated_at: DateTime<Utc>",
          deleted_at as "deleted_at: DateTime<Utc>"
         FROM videos_transcriptions vt
        WHERE video_id = $1
        "#,
        video_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(transcription);
}

pub async fn create(pool: &PgPool, dto: CreateTranscriptionDto) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos_transcriptions (video_id, transcription_id, transcriber_id)
        VALUES ($1, $2, $3);
        "#,
        dto.video_id,
        dto.transcription_id,
        dto.transcriber_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
#[cfg(test)]
mod test {

    use std::str::FromStr;

    use sqlx::PgPool;

    use crate::database::queries::{subtitling::find_by_video_id, video::find_by_transcription_id};

    #[sqlx::test(migrations = "../migrations", fixtures("videos"))]
    async fn test_create_transcription(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();

        let dto = super::CreateTranscriptionDto {
            video_id: id,
            transcription_id: "Transcription_Test_Ok".to_string(),
            transcriber_id: 1,
        };

        let result = super::create(&pool, dto).await;
        assert!(result.is_ok());

        let count = sqlx::query!(
            "SELECT COUNT(*) FROM videos_transcriptions where video_id = $1",
            id
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(count.count.is_some());
        assert_eq!(count.count.unwrap(), 1);
    }

    #[sqlx::test(migrations = "../migrations")]

    async fn test_create_transcription_if_foreign_key_video(pool: PgPool) {
        let id = uuid::Uuid::new_v4();

        let dto = super::CreateTranscriptionDto {
            video_id: id,
            transcription_id: "Transcription_Test_Err".to_string(),
            transcriber_id: 1,
        };

        let result = super::create(&pool, dto).await;

        assert!(result.is_err());
    }
    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "videos_transcriptions")
    )]
    async fn test_find_by_transcription_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let transcription_id = "Transcription_Test_Ok";

        let find_sucess = find_by_transcription_id(&pool, transcription_id)
            .await
            .unwrap();

        assert_eq!(find_sucess.id, id);
    }

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "videos_transcriptions")
    )]
    async fn test_not_found_by_transcription_id(pool: PgPool) {
        let transcription_id = "Transcription_Test_Err";
        let find_not_sucess = find_by_transcription_id(&pool, transcription_id).await;

        assert!(find_not_sucess.is_err());
    }

    #[sqlx::test(
        migrations = "../migrations",
        fixtures("videos", "videos_transcriptions")
    )]
    async fn test_find_transcription_by_video_id(pool: PgPool) {
        let id = uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap();
        let find_success = super::find_by_video_id(&pool, &id).await;

        assert!(find_success.is_ok());
    }
}
