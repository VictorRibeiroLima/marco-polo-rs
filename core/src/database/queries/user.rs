use sqlx::PgPool;

use crate::database::models::user::{User, UserRole};
use chrono::{DateTime, Utc};

pub struct CreateUserDto<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: &'a Option<UserRole>,
}

pub async fn create(pool: &PgPool, dto: CreateUserDto<'_>) -> Result<(), sqlx::Error> {
    let password = bcrypt::hash(dto.password, bcrypt::DEFAULT_COST).unwrap();

    sqlx::query!(
        r#"
        INSERT INTO users (name, email, password, role)
        VALUES ($1, $2, $3, $4)
        "#,
        dto.name,
        dto.email,
        password,
        dto.role as &Option<UserRole>,
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
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>",
            deleted_at as "deleted_at: DateTime<Utc>"
        
        FROM users WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    return Ok(user);
}
