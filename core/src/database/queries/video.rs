use chrono::NaiveDateTime;

use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::{
    video::{Video, VideoStage, VideoWithStorage, VideoWithStorageAndChannel},
    video_storage::StorageVideoStage,
};

use super::{filter::Filter, macros::find_all, pagination::Pagination, storage};

pub struct CreateVideoDto<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub channel_id: i32,
    pub language: &'a str,
    pub tags: Option<&'a str>,
    pub start_time: &'a str,
    pub original_url: &'a str,
}

pub struct CreateErrorDto<'a> {
    pub video_id: &'a Uuid,
    pub error: &'a str,
    pub stage: VideoStage,
}

pub async fn create(pool: &PgPool, dto: CreateVideoDto<'_>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos (id, title, description, user_id, channel_id, language, start_time, original_url, tags)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);
        "#,
        dto.id,
        dto.title,
        dto.description,
        dto.user_id,
        dto.channel_id,
        dto.language,
        dto.start_time,
        dto.original_url,
        dto.tags,
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
        SET 
        stage = $1,
        updated_at = NOW()
        WHERE id = $2
        "#,
        stage as VideoStage,
        video_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_metadata(
    pool: &PgPool,
    video_id: &Uuid,
    original_duration: &str,
    end_time: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET 
        original_duration = $1,
        end_time = $2,
        updated_at = NOW()
        WHERE id = $3
        "#,
        original_duration,
        end_time,
        video_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn change_error_state(
    pool: &PgPool,
    video_id: &Uuid,
    error: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET 
        error = $1,
        updated_at = NOW()
        WHERE id = $2
        "#,
        error,
        video_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_error(pool: &PgPool, dto: CreateErrorDto<'_>) -> Result<i64, sqlx::Error> {
    let mut trx = pool.begin().await?;

    sqlx::query!(
        r#"
        UPDATE videos
        SET 
        error = true,
        updated_at = NOW()
        WHERE id = $1
    "#,
        dto.video_id
    )
    .execute(&mut trx)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO videos_errors (video_id, error, stage)
        VALUES ($1, $2, $3);
        "#,
        dto.video_id,
        dto.error,
        &dto.stage as &VideoStage,
    )
    .execute(&mut trx)
    .await?;

    let count_result = sqlx::query!(
        r#"
        SELECT COUNT(*) as "count!: i64"
        FROM videos_errors
        WHERE video_id = $1 and stage = $2"#,
        dto.video_id,
        dto.stage as VideoStage,
    )
    .fetch_optional(&mut trx)
    .await?;

    let count = count_result.map(|row| row.count).unwrap_or_default();

    trx.commit().await?;

    Ok(count)
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
            v.error,
            v.original_url,
            v.original_duration,
            v.start_time,
            v.end_time,
            v.tags,
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
            v.error,
            v.original_url,
            v.original_duration,
            v.start_time,
            v.end_time,
            v.tags,
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
    mut filter: Filter<Video>,
) -> Result<Vec<Video>, sqlx::Error> {
    let (offset, limit, order, order_by) = pagination.to_tuple();
    filter.options.user_id = Some(owner_id);
    filter.options.deleted_at = Some(None);

    let (where_sql, param_count) = filter.gen_where_statements(None);

    let sql = format!(
        r#"
        SELECT 
            *
        FROM 
            videos 
        WHERE
            {}
        ORDER BY 
            {} {}
        LIMIT
            ${}
        OFFSET 
            ${}
        "#,
        where_sql,
        order_by.name(),
        order.name(),
        param_count + 1,
        param_count + 2,
    );

    let mut query = sqlx::query_as(&sql);
    query = filter.apply(query);
    query = query.bind(limit).bind(offset);

    let videos: Vec<Video> = query.fetch_all(pool).await?;

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
