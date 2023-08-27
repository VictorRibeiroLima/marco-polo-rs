use sqlx::PgPool;
use uuid::Uuid;

use crate::database::models::{traits::FromRows, video::with::VideoWithChannels};

const BASE_QUERY: &'static str = r#"SELECT 
v.id, 
v.title,
v.description,
v.url,
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
c.id as "c.id",
c.creator_id as "c.creator_id",
c.error as "c.error",
c.name as "c.name",
c.platform as "c.platform",
c.auth as "c.auth",
c.created_at as "c.created_at",
c.updated_at as "c.updated_at",
c.deleted_at as "c.deleted_at"

FROM 
  videos v
INNER JOIN 
  videos_channels vc ON vc.video_id = v.id
INNER JOIN 
  channels c ON c.id = vc.channel_id"
"#;

pub async fn with_channels(
    pool: &PgPool,
    video_id: Uuid,
) -> Result<VideoWithChannels, sqlx::Error> {
    let query = format!("{} WHERE v.id = $1", BASE_QUERY);
    let rows = sqlx::query(&query).bind(video_id).fetch_all(pool).await?;

    let videos = VideoWithChannels::from_rows(&rows)?;

    let video = match videos.into_iter().next() {
        Some(video) => video,
        None => return Err(sqlx::Error::RowNotFound),
    };

    Ok(video)
}
