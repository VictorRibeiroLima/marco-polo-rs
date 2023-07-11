use sqlx::PgPool;

use crate::database::models::channel::{Channel, ChannelOrderFields};

use super::{
    macros::find_all,
    pagination::{Pagination, PaginationOrder},
};

pub struct UpdateChannelDto {
    pub id: i32,
    pub name: String,
    pub refresh_token: String,
}

find_all!(Channel, ChannelOrderFields::Id, "channels");

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

pub async fn find_all_by_owner(
    pool: &PgPool,
    owner_id: i32,
    pagination: Pagination<Channel>,
) -> Result<Vec<Channel>, sqlx::Error> {
    let offset = match pagination.offset {
        Some(offset) => offset,
        None => 0,
    };

    let limit = match pagination.limit {
        Some(limit) => limit,
        None => 10,
    };

    let order = match pagination.order {
        Some(order) => order,
        None => PaginationOrder::Asc,
    };

    let order_by = match pagination.order_by {
        Some(order_by) => order_by,
        None => ChannelOrderFields::Id,
    };

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
