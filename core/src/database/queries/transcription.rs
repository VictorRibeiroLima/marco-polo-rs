use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::{
    models::video_transcription::VideosTranscription, queries::video::CreateVideoDto,
};

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

    use sqlx::PgPool;

    use crate::database::queries::video::create;

    #[sqlx::test(migrations = "../migrations", fixtures("user", "channel"))]
    async fn test_create_transcription(pool: PgPool) {
        let id = uuid::Uuid::new_v4();

        let dto = super::CreateVideoDto {
            id: &id,
            title: "Test",
            description: "Test",
            user_id: 666,
            channel_id: 666,
            language: "en",
        };

        create(&pool, dto).await.unwrap();

        let dto = super::CreateTranscriptionDto {
            video_id: id,
            transcription_id: "Teste".to_string(),
            transcriber_id: 1,
        };

        match super::create(&pool, dto).await {
            Ok(_) => {
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
            Err(err) => {
                return Err(err).unwrap();
            }
        }
    }

    #[sqlx::test(migrations = "../migrations")]

    async fn test_create_transcription_if_foreign_key_video(pool: PgPool) {
        let id = uuid::Uuid::new_v4();

        let dto = super::CreateTranscriptionDto {
            video_id: id,
            transcription_id: "Teste".to_string(),
            transcriber_id: 1,
        };

        let result = super::create(&pool, dto).await;

        assert!(result.is_err());
    }
}
