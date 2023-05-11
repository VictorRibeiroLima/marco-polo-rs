use sqlx::PgPool;

use crate::database::models::user::{User, UserRole};
use chrono::{DateTime, Utc};

pub struct CreateUserDto<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: Option<&'a UserRole>,
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

#[cfg(test)]
mod test {
    use super::*;

    use sqlx::{Pool, Postgres};

    use crate::database::models::user::UserRole;

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create_with_role(pool: Pool<Postgres>) {
        let email = "test@hotmail.com";

        let user_dto = CreateUserDto {
            email: &email,
            name: "Test",
            password: "123456",
            role: Some(&UserRole::User),
        };

        create(&pool, user_dto).await.unwrap();

        let user = find_by_email(&pool, &email).await.unwrap().unwrap();

        assert_eq!(user.email, email);
        assert!(bcrypt::verify("123456", &user.password).unwrap());
    }

    #[sqlx::test(migrations = "../migrations")]
    async fn test_create_without_role(pool: Pool<Postgres>) {
        let email = "test@hotmail.com";

        let user_dto = CreateUserDto {
            email,
            name: "Test",
            password: "123456",
            role: None,
        };

        create(&pool, user_dto).await.unwrap();

        let user = find_by_email(&pool, &email).await.unwrap().unwrap();

        assert_eq!(user.email, email);
        assert!(bcrypt::verify("123456", &user.password).unwrap());
    }
}
