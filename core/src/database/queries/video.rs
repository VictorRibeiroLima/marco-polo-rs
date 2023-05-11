use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::video::Video;

pub struct CreateVideoDto<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub channel_id: i32,
    pub language: &'a str,
}

pub struct UpdateVideoTranscriptionDto {
    pub video_id: Uuid,
    pub storage_id: i32,
    pub path: String,
}

pub async fn create(pool: &PgPool, dto: CreateVideoDto<'_>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos (id, title, description, user_id, channel_id, language)
        VALUES ($1, $2, $3, $4, $5, $6);
        "#,
        dto.id,
        dto.title,
        dto.description,
        dto.user_id,
        dto.channel_id,
        dto.language,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_by_transcription_id(
    pool: &PgPool,
    transcription_id: &str,
) -> Result<Video, sqlx::Error> {
    let video = sqlx::query_as!(
        Video,
        r#"
        SELECT 
            v.id as "id: Uuid", 
            v.title,
            v.description,
            v.url,
            v.language,
            v.user_id,
            v.channel_id,
            v.created_at as "created_at: DateTime<Utc>",
            v.updated_at as "updated_at: DateTime<Utc>",
            v.deleted_at as "deleted_at: DateTime<Utc>",
            v.uploaded_at as "uploaded_at: DateTime<Utc>"
        FROM 
            videos v
        INNER JOIN 
            videos_transcriptions vt ON v.id = vt.video_id
        WHERE 
            vt.transcription_id = $1
    "#,
        transcription_id
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

pub async fn update_transcription(
    pool: &PgPool,
    dto: UpdateVideoTranscriptionDto,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE videos_transcriptions SET storage_id = $1, path = $2, updated_at = NOW()
            WHERE video_id = $3;
        "#,
        dto.storage_id,
        dto.path,
        dto.video_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use sqlx::PgPool;

    #[sqlx::test(migrations = "../migrations", fixtures("user", "channel"))]
    async fn test_create_video(pool: PgPool) {
        let id = uuid::Uuid::new_v4();

        let dto = super::CreateVideoDto {
            id: &id,
            title: "Test",
            description: "Test",
            user_id: 666,
            channel_id: 666,
            language: "en",
        };

        super::create(&pool, dto).await.unwrap();

        let count = sqlx::query!("SELECT COUNT(*) FROM videos where id = $1", id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(count.count.is_some());
        assert_eq!(count.count.unwrap(), 1);
    }
}
