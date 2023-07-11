use sqlx::PgPool;

use crate::database::models::user::{User, UserOrderFields, UserRole};
use chrono::NaiveDateTime;

use super::macros::find_all;

pub struct CreateUserDto<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: Option<&'a UserRole>,
}

find_all!(User, UserOrderFields::Id, "users");

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id,
            name, 
            email,
            password,
            role as "role: UserRole", 
            created_at as "created_at: NaiveDateTime",
            updated_at as "updated_at: NaiveDateTime",
            deleted_at as "deleted_at: NaiveDateTime"
        FROM 
            users 
        WHERE 
            id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    return Ok(user);
}

pub async fn create(pool: &PgPool, dto: CreateUserDto<'_>) -> Result<(), sqlx::Error> {
    let password = bcrypt::hash(dto.password, bcrypt::DEFAULT_COST).unwrap();
    let role = dto.role.unwrap_or(&UserRole::User);
    sqlx::query!(
        r#"
        INSERT INTO users (name, email, password, role)
        VALUES ($1, $2, $3, $4)
        "#,
        dto.name,
        dto.email,
        password,
        role as &UserRole,
    )
    .execute(pool)
    .await?;

    return Ok(());
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT  
            id,
            name,
            email,
            password,
            role as "role: UserRole",
            created_at as "created_at: NaiveDateTime",
            updated_at as "updated_at: NaiveDateTime",
            deleted_at as "deleted_at: NaiveDateTime"
        
        FROM users WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    return Ok(user);
}
