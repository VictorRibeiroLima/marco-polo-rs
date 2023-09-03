use sqlx::PgPool;

use crate::database::models::{original_video::with::OriginalVideoWithVideos, traits::FromRows};

const BASE_QUERY: &str = r#"
SELECT 
    ov.*, 
    v.id AS "v.id", 
    v.title AS "v.title", 
    v.description AS "v.description", 
    v.user_id AS "v.user_id", 
    v.channel_id AS "v.channel_id", 
    v.url AS "v.url", 
    v.language AS "v.language", 
    v.stage AS "v.stage", 
    v.error AS "v.error", 
    v.original_video_id AS "v.original_video_id", 
    v.start_time AS "v.start_time", 
    v.end_time AS "v.end_time", 
    v.tags AS "v.tags", 
    v.created_at AS "v.created_at", 
    v.updated_at AS "v.updated_at", 
    v.deleted_at AS "v.deleted_at", 
    v.uploaded_at AS "v.uploaded_at"
FROM 
  original_videos ov
INNER JOIN 
  videos v 
ON 
  ov.id = v.original_video_id
"#;

pub async fn find_with_videos(
    pool: &PgPool,
    id: i32,
) -> Result<OriginalVideoWithVideos, sqlx::Error> {
    let query = format!(
        r#"
        {}
        WHERE ov.id = $1
        "#,
        BASE_QUERY
    );

    let rows = sqlx::query(&query).bind(id).fetch_all(pool).await?; // fetch_all because we expect multiple rows

    let value = OriginalVideoWithVideos::from_rows(&rows)?;

    let value = match value.into_iter().next() {
        Some(value) => value,
        None => return Err(sqlx::Error::RowNotFound),
    };
    Ok(value)
}
