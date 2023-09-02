use std::ops::DerefMut;

use chrono::NaiveDateTime;

use sqlx::{Acquire, PgExecutor, PgPool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::database::models::{
    video::{stage::VideoStage, with::VideoWithStorage, Video},
    video_storage::StorageVideoStage,
};

use super::{filter::Filter, macros::find_all, pagination::Pagination, storage};

pub mod with_channel;
pub mod with_original;

pub struct CreateVideoDto<'a> {
    pub id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub channel_ids: &'a [i32],
    pub language: &'a str,
    pub tags: Option<String>,
    pub start_time: &'a str,
    pub end_time: Option<&'a str>,
    pub original_id: i32,
}

pub struct CreateErrorsDto<'a> {
    pub video_ids: Vec<Uuid>,
    pub error: &'a str,
}

pub async fn create(
    pool: impl PgExecutor<'_> + Acquire<'_, Database = Postgres>,
    dto: CreateVideoDto<'_>,
) -> Result<(), sqlx::Error> {
    let mut trx = pool.begin().await?;
    let id = sqlx::query!(
        r#"
        INSERT INTO videos (id, title, description, user_id, language, start_time, original_video_id, tags,end_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id;
        "#,
        dto.id,
        dto.title,
        dto.description,
        dto.user_id,
        dto.language,
        dto.start_time,
        dto.original_id,
        dto.tags,
        dto.end_time,
    )
    .fetch_one(&mut *trx)
    .await?;

    let mut query_builder =
        QueryBuilder::new("INSERT INTO videos_channels (video_id, channel_id) ");

    query_builder.push_values(dto.channel_ids, |mut builder, channel_id| {
        builder.push_bind(id.id).push_bind(channel_id);
    });

    let insert_query = query_builder.build();

    insert_query.execute(&mut *trx).await?;

    trx.commit().await?;

    Ok(())
}

pub async fn create_many(
    pool: impl PgExecutor<'_> + Acquire<'_, Database = Postgres>,
    dtos: Vec<CreateVideoDto<'_>>,
) -> Result<(), sqlx::Error> {
    let mut trx = pool.begin().await?;
    let mut query_builder = QueryBuilder::new(
        "INSERT INTO videos (id, title, description, user_id, language, start_time, original_video_id, tags,end_time) ",
    );

    query_builder.push_values(&dtos, |mut builder, dto| {
        builder
            .push_bind(dto.id)
            .push_bind(dto.title)
            .push_bind(dto.description)
            .push_bind(dto.user_id)
            .push_bind(dto.language)
            .push_bind(dto.start_time)
            .push_bind(dto.original_id)
            .push_bind(&dto.tags)
            .push_bind(dto.end_time);
    });

    query_builder.push(" RETURNING id");

    let insert_query = query_builder.build();

    //postgres should return the ids in the same order as the dtos
    let ids = insert_query.fetch_all(trx.deref_mut()).await?;

    //So we can zip them together
    let mut video_channels_ids: Vec<(Uuid, i32)> = vec![];

    for (dto, id) in dtos.into_iter().zip(ids.into_iter()) {
        for channel_id in dto.channel_ids {
            let id = id.try_get::<Uuid, _>("id")?;
            video_channels_ids.push((id, *channel_id));
        }
    }

    let mut query_builder =
        QueryBuilder::new("INSERT INTO videos_channels (video_id, channel_id) ");

    query_builder.push_values(
        &video_channels_ids,
        |mut builder, (video_id, channel_id)| {
            builder.push_bind(video_id).push_bind(channel_id);
        },
    );

    let insert_query = query_builder.build();

    insert_query.execute(trx.deref_mut()).await?;

    trx.commit().await?;

    Ok(())
}

pub async fn change_stage(
    pool: impl PgExecutor<'_>,
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
            v.language,
            v.user_id,
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
            v.language,
            v.user_id,
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
