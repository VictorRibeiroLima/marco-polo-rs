use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::database::models::channel::Channel;

pub struct UpdateChannelDto {
    pub id: i32,
    pub name: String,
    pub refresh_token: String,
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Channel, sqlx::Error> {
    let channel = sqlx::query_as!(
        Channel,
        r#"
        SELECT 
            id,
            name,
            csrf_token,
            refresh_token,
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

pub async fn create(pool: &PgPool, csrf_token: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO channels (csrf_token) 
    VALUES ($1)
    "#,
        csrf_token
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_by_csrf_token(pool: &PgPool, csrf_token: String) -> Result<Channel, sqlx::Error> {
    let channel = sqlx::query_as!(
        Channel,
        r#"
        SELECT 
            id,
            name,
            csrf_token,
            refresh_token,
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>",
            deleted_at as "deleted_at: DateTime<Utc>"
        FROM channels WHERE csrf_token = $1
        "#,
        csrf_token
    )
    .fetch_one(pool)
    .await?;

    return Ok(channel);
}

pub async fn update(pool: &PgPool, dto: UpdateChannelDto) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    UPDATE channels SET 
        name = $1,
        refresh_token = $2,
        csrf_token = NULL,
        updated_at = NOW()
    WHERE id = $3
    "#,
        dto.name,
        dto.refresh_token,
        dto.id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod test {

    use sqlx::PgPool;

    use crate::database::queries::channel::{create, find_by_id};

    const CSRF_TOKEN: &str = "123has_iuf12134";

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create(pool: PgPool) {
        let result = create(&pool, CSRF_TOKEN.to_string()).await;

        assert!(result.is_ok());
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) FROM channels WHERE csrf_token = $1
        "#,
            CSRF_TOKEN
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(record.count.is_some());

        assert_eq!(record.count.unwrap(), 1);
    }

    #[sqlx::test(migrations = "../migrations", fixtures("channel"))]
    async fn test_find_by_id(pool: PgPool) {
        let channel_id = 666;
        let find_success = find_by_id(&pool, channel_id).await;

        assert!(find_success.is_ok());
        assert_eq!(find_success.unwrap().id, channel_id);
    }

    #[sqlx::test(migrations = "../migrations", fixtures("channel"))]
    async fn test_not_find_by_id(pool: PgPool) {
        let invalid_channel_id = 999;
        let find_error = find_by_id(&pool, invalid_channel_id).await;
        assert!(find_error.is_err());
    }
}
