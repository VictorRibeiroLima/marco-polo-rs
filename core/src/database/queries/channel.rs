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
            creator_id,
            refresh_token,
            created_at as "created_at: chrono::NaiveDateTime",
            updated_at as "updated_at: chrono::NaiveDateTime",
            deleted_at as "deleted_at: chrono::NaiveDateTime"
        FROM channels WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    return Ok(channel);
}

pub async fn create(pool: &PgPool, csrf_token: String, creator_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO channels (csrf_token,creator_id) 
    VALUES ($1,$2)
    "#,
        csrf_token,
        creator_id
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
            creator_id,
            csrf_token,
            refresh_token,
            created_at as "created_at: chrono::NaiveDateTime",
            updated_at as "updated_at: chrono::NaiveDateTime",
            deleted_at as "deleted_at: chrono::NaiveDateTime"
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

    #[sqlx::test(migrations = "../migrations", fixtures("user"))]
    async fn test_create(pool: PgPool) {
        let fixture_user_id = 666;
        let result = create(&pool, CSRF_TOKEN.to_string(), fixture_user_id).await;

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
