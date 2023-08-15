use chrono::NaiveDateTime;

use sqlx::{postgres::PgRow, FromRow, PgExecutor, PgPool, QueryBuilder};
use uuid::Uuid;

use crate::database::models::{
    original_video::OriginalVideo,
    video::{
        stage::VideoStage,
        with::{VideoWithOriginal, VideoWithStorage, VideoWithStorageAndChannel},
        Video,
    },
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
    pub end_time: Option<&'a str>,
    pub original_id: i32,
}

pub struct CreateErrorsDto<'a> {
    pub video_ids: Vec<Uuid>,
    pub error: &'a str,
}

pub async fn create(pool: impl PgExecutor<'_>, dto: CreateVideoDto<'_>) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO videos (id, title, description, user_id, channel_id, language, start_time, original_video_id, tags,end_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,$10);
        "#,
        dto.id,
        dto.title,
        dto.description,
        dto.user_id,
        dto.channel_id,
        dto.language,
        dto.start_time,
        dto.original_id,
        dto.tags,
        dto.end_time,
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
        updated_at = NOW(),
        error = false
        WHERE id = $2
        "#,
        stage as VideoStage,
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

//This method is assuming that every video has the same stage. maybe we should change this later
pub async fn create_errors(pool: &PgPool, dto: CreateErrorsDto<'_>) -> Result<i64, sqlx::Error> {
    let mut trx = pool.begin().await?;
    let video_ids = dto.video_ids;
    let video_id = video_ids.first().ok_or(sqlx::Error::RowNotFound)?;

    println!("{:?}", video_ids);

    //Postgres hack see:https://github.com/launchbadge/sqlx/blob/main/FAQ.md
    let result = sqlx::query!(
        r#"
        UPDATE videos
        SET 
        error = true,
        updated_at = NOW()
        WHERE id = ANY($1)
        RETURNING stage as "stage: VideoStage"
    "#,
        &video_ids[..],
    )
    .fetch_one(&mut *trx)
    .await?;

    let stage = result.stage;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO videos_errors (video_id, error, stage) ");

    query_builder.push_values(&video_ids, |mut builder, id| {
        builder
            .push_bind(id)
            .push_bind(dto.error)
            .push_bind(&stage as &VideoStage);
    });

    let insert_query = query_builder.build();
    insert_query.execute(&mut *trx).await?;

    let count_result = sqlx::query!(
        r#"
        SELECT COUNT(*) as "count!: i64"
        FROM videos_errors
        WHERE video_id = $1 and stage = $2"#,
        video_id,
        stage as VideoStage,
    )
    .fetch_optional(&mut *trx)
    .await?;

    let count = count_result.map(|row| row.count).unwrap_or_default();

    trx.commit().await?;

    Ok(count)
}

pub async fn set_url(pool: &PgPool, video_id: &Uuid, url: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET url = $1, stage = 'DONE', uploaded_at = NOW(), error = false
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
            v.original_video_id,
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
            v.original_video_id,
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

pub async fn find_with_original(
    pool: &PgPool,
    id: &Uuid,
) -> Result<VideoWithOriginal, sqlx::Error> {
    let video = sqlx::query_as(
        r#"
        SELECT 
            v.id, 
            v.title,
            v.description,
            v.url,
            v.language,
            v.user_id,
            v.channel_id,
            v.error,
            v.original_video_id,
            v.start_time,
            v.end_time,
            v.tags,
            v.stage,
            v.created_at,
            v.updated_at,
            v.deleted_at,
            v.uploaded_at,
            ov.url as "ov.url",
            ov.id as "ov.id",
            ov.duration as "ov.duration",
            ov.created_at as "ov.created_at",
            ov.updated_at as "ov.updated_at"
        FROM 
            videos v
        INNER JOIN 
            original_videos ov ON v.original_video_id = ov.id
        WHERE 
            v.id = $1 AND deleted_at IS NULL
    "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    return Ok(video);
}

pub async fn find_all_with_original(
    pool: &PgPool,
    pagination: Pagination<Video>,
    video_filter: Filter<Video>,
    original_video_filter: Filter<OriginalVideo>,
) -> Result<Vec<VideoWithOriginal>, sqlx::Error> {
    let (offset, limit, order, order_by) = pagination.to_tuple();

    let (video_where, video_param_count) = video_filter.gen_where_statements_with_alias("v", None);

    let (original_video_where, _) =
        original_video_filter.gen_where_statements_with_alias("ov", Some(video_param_count));

    let query_where: String;
    if video_where != "" && original_video_where != "" {
        query_where = format!("{} AND {}", video_where, original_video_where);
    } else if video_where != "" {
        query_where = video_where;
    } else if original_video_where != "" {
        query_where = original_video_where;
    } else {
        query_where = "".to_string();
    }

    let mut sql = format!(
        r#"
        SELECT
            v.id, 
            v.title,
            v.description,
            v.url,
            v.language,
            v.user_id,
            v.channel_id,
            v.error,
            v.original_video_id,
            v.start_time,
            v.end_time,
            v.tags,
            v.stage,
            v.created_at,
            v.updated_at,
            v.deleted_at,
            v.uploaded_at,
            ov.url as "ov.url",
            ov.id as "ov.id",
            ov.duration as "ov.duration",
            ov.created_at as "ov.created_at",
            ov.updated_at as "ov.updated_at"
            FROM
                videos v
            INNER JOIN
                original_videos ov ON v.original_video_id = ov.id           
        "#,
    );

    if query_where != "" {
        sql = format!("{} WHERE {}", sql, query_where);
    }

    sql = format!(
        r#"{} ORDER BY 
            v.{} {}
        LIMIT
            {}
        OFFSET 
            {}"#,
        sql,
        order_by.name(),
        order.name(),
        limit,
        offset
    );

    let mut videos: Vec<VideoWithOriginal> = vec![];

    let mut query = sqlx::query(&sql);
    query = video_filter.apply_raw(query);
    query = original_video_filter.apply_raw(query);

    let rows: Vec<PgRow> = query.fetch_all(pool).await?;
    for row in rows {
        let video: VideoWithOriginal = VideoWithOriginal::from_row(&row)?;
        videos.push(video);
    }
    return Ok(videos);
}

pub async fn bulk_update_end_time(
    pool: impl PgExecutor<'_>,
    ids: Vec<Uuid>,
    end_time: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE videos
        SET 
        end_time = $1,
        updated_at = NOW()
        WHERE id = ANY($2)
        "#,
        end_time,
        &ids[..],
    )
    .execute(pool)
    .await?;

    Ok(())
}
