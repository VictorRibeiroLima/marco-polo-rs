use sqlx::{postgres::PgRow, FromRow, PgPool};
use uuid::Uuid;

use crate::database::{
    models::{
        self,
        original_video::OriginalVideo,
        traits::FromRows,
        video::{
            with::{VideoWithOriginal, VideoWithOriginalAndVideoChannels},
            Video,
        },
    },
    queries::{filter::Filter, pagination::Pagination},
};

const BASE_SELECT: &'static str = r#"SELECT 
v.id, 
v.title,
v.description,
v.language,
v.user_id,
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
"#;

const BASE_FROM: &'static str = r#"

FROM 
videos v
INNER JOIN 
original_videos ov ON v.original_video_id = ov.id"#;

pub async fn find_with_original(
    pool: &PgPool,
    id: &Uuid,
) -> Result<VideoWithOriginal, sqlx::Error> {
    let query = format!(
        "{} {} WHERE v.id = $1 AND v.deleted_at IS NULL",
        BASE_SELECT, BASE_FROM
    );

    let video = sqlx::query_as(&query).bind(id).fetch_one(pool).await?;

    return Ok(video);
}

//TODO: test
pub async fn find_with_original_and_video_channels(
    pool: &PgPool,
    id: &Uuid,
) -> Result<VideoWithOriginalAndVideoChannels, sqlx::Error> {
    let select = format!("{},{}", BASE_SELECT, models::video_channel::ALIAS_COLUMNS);

    let from = format!(
        r#"{} 
        INNER JOIN 
            videos_channels vc ON vc.video_id = v.id"#,
        BASE_FROM
    );

    let query = format!(
        "{} {} WHERE v.id = $1 AND v.deleted_at IS NULL",
        select, from
    );

    let video = sqlx::query(&query).bind(id).fetch_all(pool).await?;

    let videos = VideoWithOriginalAndVideoChannels::from_rows(&video)?;

    let video = match videos.into_iter().next() {
        Some(video) => video,
        None => return Err(sqlx::Error::RowNotFound),
    };

    return Ok(video);
}

pub async fn find_by_user_id_with_original(
    pool: &PgPool,
    id: &Uuid,
    user_id: i32,
) -> Result<VideoWithOriginal, sqlx::Error> {
    let query = format!(
        "{} {} WHERE v.id = $1 AND v.user_id = $2 AND v.deleted_at IS NULL",
        BASE_SELECT, BASE_FROM
    );

    let video = sqlx::query_as(&query)
        .bind(id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    return Ok(video);
}

//TODO: test
pub async fn find_by_user_id_with_original_and_video_channels(
    pool: &PgPool,
    id: &Uuid,
    user_id: i32,
) -> Result<VideoWithOriginalAndVideoChannels, sqlx::Error> {
    let select = format!("{},{}", BASE_SELECT, models::video_channel::ALIAS_COLUMNS);

    let from = format!(
        r#"{} 
        INNER JOIN 
            videos_channels vc ON vc.video_id = v.id"#,
        BASE_FROM
    );

    let query = format!(
        "{} {} WHERE v.id = $1 AND v.user_id = $2 AND v.deleted_at IS NULL",
        select, from
    );

    let video = sqlx::query(&query)
        .bind(id)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    let videos = VideoWithOriginalAndVideoChannels::from_rows(&video)?;

    let video = match videos.into_iter().next() {
        Some(video) => video,
        None => return Err(sqlx::Error::RowNotFound),
    };

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

    let mut sql = format!("{} {}", BASE_SELECT, BASE_FROM);

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

pub async fn find_all_with_original_by_ids(
    pool: &PgPool,
    ids: Vec<Uuid>,
) -> Result<Vec<VideoWithOriginal>, sqlx::Error> {
    let query = format!("{} {} WHERE v.id = ANY($1)", BASE_SELECT, BASE_FROM);

    let rows = sqlx::query(&query).bind(&ids[..]).fetch_all(pool).await?;

    let mut videos = vec![];
    for row in rows {
        let video: VideoWithOriginal = VideoWithOriginal::from_row(&row)?;
        videos.push(video);
    }
    return Ok(videos);
}
