use sqlx::PgPool;

use crate::database::models::channel::auth::data::Oath2Data;
use crate::database::models::channel::auth::AuthType;
use crate::database::models::channel::{platform::Platform, Channel};

use super::{macros::find_all, pagination::Pagination};

pub struct UpdateChannelDto {
    pub id: i32,
    pub name: String,
    pub refresh_token: String,
}

pub struct CreateChannelDto {
    pub auth: AuthType,
    pub creator_id: i32,
    pub platform: Platform,
}

find_all!(Channel, "channels");

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Channel, sqlx::Error> {
    let channel = sqlx::query_as!(
        Channel,
        r#"
        SELECT 
            id,
            name,
            creator_id,
            error,
            platform as "platform: Platform",
            auth as "auth: sqlx::types::Json<AuthType>",
            created_at as "created_at: chrono::NaiveDateTime",
            updated_at as "updated_at: chrono::NaiveDateTime",
            deleted_at as "deleted_at: chrono::NaiveDateTime"
        FROM channels WHERE id = $1 AND deleted_at IS NULL AND error = FALSE
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    return Ok(channel);
}

pub async fn find_by_and_creator(
    pool: &PgPool,
    id: i32,
    creator_id: i32,
) -> Result<Channel, sqlx::Error> {
    let channel = sqlx::query_as!(
        Channel,
        r#"
        SELECT 
            id,
            name,
            creator_id,
            error,
            platform as "platform: Platform",
            auth as "auth: sqlx::types::Json<AuthType>",
            created_at as "created_at: chrono::NaiveDateTime",
            updated_at as "updated_at: chrono::NaiveDateTime",
            deleted_at as "deleted_at: chrono::NaiveDateTime"
        FROM channels WHERE id = $1 AND creator_id = $2 AND deleted_at IS NULL
        "#,
        id,
        creator_id
    )
    .fetch_one(pool)
    .await?;

    return Ok(channel);
}

pub async fn change_error_state(pool: &PgPool, id: i32, error: bool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    UPDATE channels SET 
        error = $1,
        updated_at = NOW()
    WHERE id = $2
    "#,
        error,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create(pool: &PgPool, dto: CreateChannelDto) -> Result<(), sqlx::Error> {
    let json = serde_json::to_value(dto.auth).unwrap();

    sqlx::query!(
        r#"
    INSERT INTO channels (auth,creator_id,platform) 
    VALUES ($1,$2,$3)
    "#,
        json,
        dto.creator_id,
        dto.platform as Platform
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_token(
    pool: &PgPool,
    csrf_token: String,
    channel_id: i32,
) -> Result<(), sqlx::Error> {
    let auth_type = AuthType::Oauth2(Oath2Data {
        csrf_token: Some(csrf_token),
        refresh_token: None,
    });

    let json = serde_json::to_value(auth_type).unwrap();

    sqlx::query!(
        r#"
    UPDATE channels SET 
        auth = $1,
        updated_at = NOW()
    WHERE id = $2
    "#,
        json,
        channel_id
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
            error,
            platform as "platform: Platform",
            auth as "auth: sqlx::types::Json<AuthType>",
            created_at as "created_at: chrono::NaiveDateTime",
            updated_at as "updated_at: chrono::NaiveDateTime",
            deleted_at as "deleted_at: chrono::NaiveDateTime"
        FROM channels WHERE auth -> 'data' ->> 'csrf_token' = $1
        "#,
        csrf_token
    )
    .fetch_one(pool)
    .await?;

    return Ok(channel);
}

pub async fn update(pool: &PgPool, dto: UpdateChannelDto) -> Result<(), sqlx::Error> {
    let auth_type = AuthType::Oauth2(Oath2Data {
        csrf_token: None,
        refresh_token: Some(dto.refresh_token),
    });

    let json = serde_json::to_value(auth_type).unwrap();

    sqlx::query!(
        r#"
      UPDATE channels
      SET 
          name = $1,
          auth = $2,
          error = false,
          updated_at = NOW()
      WHERE id = $3
    "#,
        dto.name,
        json,
        dto.id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_all_by_owner(
    pool: &PgPool,
    owner_id: i32,
    pagination: Pagination<Channel>,
) -> Result<Vec<Channel>, sqlx::Error> {
    let (offset, limit, order, order_by) = pagination.to_tuple();

    let sql = format!(
        r#"
        SELECT 
            *
        FROM 
            channels 
        WHERE
            creator_id = $1 AND deleted_at IS NULL
        ORDER BY 
            {} {}
        LIMIT
            $2
        OFFSET 
            $3
        "#,
        order_by.name(),
        order.name()
    );

    let channels: Vec<Channel> = sqlx::query_as(&sql)
        .bind(owner_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    return Ok(channels);
}
