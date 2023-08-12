use sqlx::PgPool;

use crate::database::models::user::{User, UserRole};
use chrono::NaiveDateTime;

use super::macros::find_all;

pub struct CreateUserDto<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: Option<&'a UserRole>,
}

find_all!(User, "users");

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
            deleted_at as "deleted_at: NaiveDateTime",
            forgot_token,
            forgot_token_expires_at
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
            deleted_at as "deleted_at: NaiveDateTime",
            forgot_token,
            forgot_token_expires_at
        
        FROM users WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    return Ok(user);
}

pub async fn update_forgot_token(
    pool: &PgPool,
    id: i32,
    token: Option<impl Into<String>>,
) -> Result<(), sqlx::Error> {
    let token: Option<String> = match token {
        Some(token) => {
            let token = token.into();
            let token = bcrypt::hash(token, bcrypt::DEFAULT_COST).unwrap();
            Some(token)
        }
        None => None,
    };

    let expires_at: Option<NaiveDateTime> = match token {
        Some(_) => Some(chrono::Utc::now().naive_utc() + chrono::Duration::hours(24)),
        None => None,
    };

    sqlx::query!(
        r#"
        UPDATE users SET forgot_token = $1, forgot_token_expires_at = $2, updated_at=NOW() WHERE id = $3
        "#,
        token,
        expires_at,
        id
    )
    .execute(pool)
    .await?;

    return Ok(());
}

pub async fn update_password(pool: &PgPool, id: i32, password: &str) -> Result<(), sqlx::Error> {
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    sqlx::query!(
        r#"
        UPDATE users SET password = $1, updated_at=NOW(), forgot_token = NULL, forgot_token_expires_at = NULL WHERE id = $2
        "#,
        password,
        id
    )
    .execute(pool)
    .await?;

    return Ok(());
}

pub async fn find_by_forgot_token(
    pool: &PgPool,
    forgot_token: &str,
) -> Result<Option<User>, sqlx::Error> {
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
            deleted_at as "deleted_at: NaiveDateTime",
            forgot_token,
            forgot_token_expires_at
        
        FROM users WHERE forgot_token = $1 and forgot_token_expires_at > NOW()
        "#,
        forgot_token
    )
    .fetch_optional(pool)
    .await?;

    return Ok(user);
}
