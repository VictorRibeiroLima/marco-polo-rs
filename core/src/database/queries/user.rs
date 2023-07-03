use sqlx::PgPool;

use crate::database::models::user::{User, UserRole};
use chrono::{DateTime, Utc};

pub struct CreateUserDto<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub role: Option<&'a UserRole>,
}

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
            created_at as "created_at: DateTime<Utc>",
            updated_at as "updated_at: DateTime<Utc>",
            deleted_at as "deleted_at: DateTime<Utc>"
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
    use core::panic;

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

    #[sqlx::test(migrations = "../migrations", fixtures("user"))]
    async fn test_find_by_id(pool: Pool<Postgres>) {
        let id = 666;

        let user_result = find_by_id(&pool, id).await;
        assert!(user_result.is_ok());

        let user = user_result.unwrap();

        assert_eq!(user.id, id);
    }

    #[sqlx::test(migrations = "../migrations", fixtures("user"))]
    async fn test_not_find_by_id(pool: Pool<Postgres>) {
        let id = 665;

        let user_result = find_by_id(&pool, id).await;
        let err2 = match user_result {
            Ok(_) => panic!("User should not be found"),
            Err(err) => err,
        };

        match err2 {
            sqlx::Error::RowNotFound => {}
            _ => panic!("Expected Row not found error"),
        };
    }

    #[sqlx::test(migrations = "../migrations", fixtures("user"))]
    async fn test_not_find_by_id_deleted_at(pool: Pool<Postgres>) {
        let id = 667;

        let user_result = find_by_id(&pool, id).await;
        let err2 = match user_result {
            Ok(_) => panic!("User should not be found"),
            Err(err) => err,
        };

        match err2 {
            sqlx::Error::RowNotFound => {}
            _ => panic!("Expected Row not found error"),
        };
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
    #[sqlx::test(migrations = "../migrations", fixtures("user"))]
    async fn test_find_email(pool: PgPool) {
        let email = "teste@gmail.com";
        let find_success = find_by_email(&pool, email).await.unwrap();

        assert_eq!(find_success.unwrap().email, email);
    }

    #[sqlx::test(migrations = "../migrations", fixtures("user"))]
    async fn test_not_find_email(pool: PgPool) {
        let email = "invalid@gmail.com";
        let find_result = find_by_email(&pool, email).await.unwrap();

        assert!(find_result.is_none());
    }
}
