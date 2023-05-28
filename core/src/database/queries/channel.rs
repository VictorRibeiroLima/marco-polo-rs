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

pub async fn create(pool: &PgPool, name: String) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO channels (name) 
    VALUES ($1)
    "#,
        name
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod test {

    use sqlx::PgPool;

    use crate::database::queries::channel::{create, find_by_id};

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create(pool: PgPool) {
        let result = create(&pool, "ElonMuskCortes".to_string()).await;

        assert!(result.is_ok());
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) FROM channels WHERE name = 'ElonMuskCortes'
        "#
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
