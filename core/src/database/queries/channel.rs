use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::database::models::channel::Channel;

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Channel, sqlx::Error> {
    let channel = sqlx::query_as!(
        Channel,
        r#"
        SELECT 
            id,
            name,
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>",
            deleted_at as "deleted_at: DateTime<Utc>"
        FROM channels WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    return Ok(channel);
}
