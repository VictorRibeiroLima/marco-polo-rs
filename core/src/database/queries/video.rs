use chrono::NaiveDateTime;

use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::{
    video::{Video, VideoStage, VideoWithStorage, VideoWithStorageAndChannel},
    video_storage::StorageVideoStage,
};

use super::{macros::find_all, pagination::Pagination, storage};

pub struct CreateVideoDto<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub channel_id: i32,
    pub language: &'a str,
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

pub async fn change_stage(
    pool: &PgPool,
    video_id: &Uuid,
    stage: VideoStage,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET stage = $1
        WHERE id = $2
        "#,
        stage as VideoStage,
        video_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_url(pool: &PgPool, video_id: &Uuid, url: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET url = $1, stage = 'DONE', uploaded_at = NOW()
        WHERE id = $2
        "#,
        url,
        video_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

find_all!(Video, "videos");

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
            v.stage as "stage: VideoStage",
            v.created_at as "created_at: NaiveDateTime",
            v.updated_at as "updated_at: NaiveDateTime",
            v.deleted_at as "deleted_at: NaiveDateTime",
            v.uploaded_at as "uploaded_at: NaiveDateTime"
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

pub async fn find_by_id(pool: &PgPool, id: &Uuid) -> Result<Video, sqlx::Error> {
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
            v.stage as "stage: VideoStage",
            v.created_at as "created_at: NaiveDateTime",
            v.updated_at as "updated_at: NaiveDateTime",
            v.deleted_at as "deleted_at: NaiveDateTime",
            v.uploaded_at as "uploaded_at: NaiveDateTime"
        FROM 
            videos v
        WHERE 
            v.id = $1 AND deleted_at IS NULL
    "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(video)
}

pub async fn find_by_id_with_storage(
    pool: &PgPool,
    id: &Uuid,
    video_stage: StorageVideoStage,
) -> Result<VideoWithStorage, sqlx::Error> {
    let video = find_by_id(pool, id).await?;
    let storage = storage::find_by_video_id_and_stage(pool, id, video_stage).await?;

    Ok(VideoWithStorage { video, storage })
}

pub async fn find_all_by_owner(
    pool: &PgPool,
    owner_id: i32,
    pagination: Pagination<Video>,
) -> Result<Vec<Video>, sqlx::Error> {
    let (offset, limit, order, order_by) = pagination.to_tuple();

    let sql = format!(
        r#"
        SELECT 
            *
        FROM 
            videos 
        WHERE
            user_id = $1 AND deleted_at IS NULL
        ORDER BY 
            {} {}
        LIMIT
            $2
        OFFSET 
            $3
        "#,
        order_by.name(),
        order.name()
    );

    let videos: Vec<Video> = sqlx::query_as(&sql)
        .bind(owner_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    return Ok(videos);
}
pub async fn find_by_id_with_storage_and_channel(
    pool: &PgPool,
    id: &Uuid,
    video_stage: StorageVideoStage,
) -> Result<VideoWithStorageAndChannel, sqlx::Error> {
    let video_with_storage = find_by_id_with_storage(pool, id, video_stage).await?;
    let channel =
        crate::database::queries::channel::find_by_id(pool, video_with_storage.video.channel_id)
            .await?;

    let video_with_channel = VideoWithStorageAndChannel {
        video: video_with_storage.video,
        storage: video_with_storage.storage,
        channel,
    };
    return Ok(video_with_channel);
}
