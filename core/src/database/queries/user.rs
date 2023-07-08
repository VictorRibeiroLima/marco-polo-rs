use sqlx::PgPool;

use crate::database::models::user::{User, UserOrderFields, UserRole};
use chrono::NaiveDateTime;

use super::pagination::{Pagination, PaginationOrder};

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

pub async fn find_all(
    pool: &PgPool,
    pagination: Pagination<User>,
) -> Result<Vec<User>, sqlx::Error> {
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
        None => UserOrderFields::Id,
    };

    let sql = format!(
        r#"
        SELECT *
        FROM 
            users 
        ORDER BY 
            {} {}
        LIMIT
            $1
        OFFSET 
            $2
        "#,
        order_by.name(),
        order.name()
    );

    let all_user: Vec<User> = sqlx::query_as(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    return Ok(all_user);
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

    #[sqlx::test(migrations = "../migrations", fixtures("users"))]
    async fn test_find_all(pool: Pool<Postgres>) {
        let pagination = Pagination {
            offset: None,
            limit: None,
            order_by: None,
            order: None,
        };
        let user_result = find_all(&pool, pagination).await;
        let users = user_result.unwrap();
        let user_length = users.len();

        assert_eq!(user_length, 10)
    }

    #[sqlx::test(migrations = "../migrations", fixtures("users"))]
    async fn test_find_all_limit_20(pool: Pool<Postgres>) {
        let pagination = Pagination {
            offset: None,
            limit: Some(20),
            order_by: None,
            order: None,
        };
        let user_result = find_all(&pool, pagination).await;
        let users = user_result.unwrap();
        let user_length = users.len();

        assert_eq!(user_length, 20)
    }

    #[sqlx::test(migrations = "../migrations", fixtures("users"))]
    async fn test_find_all_order_by_id_desc(pool: Pool<Postgres>) {
        let pagination = Pagination {
            offset: None,
            limit: None,
            order_by: Some(UserOrderFields::Id),
            order: Some(PaginationOrder::Desc),
        };
        let user_result = find_all(&pool, pagination).await;
        let users = user_result.unwrap();
        let user_length = users.len();

        assert_eq!(user_length, 10);
        let mut base_id: i32 = 0;
        for (index, user) in users.into_iter().enumerate() {
            if index == 0 {
                base_id = user.id;
            } else {
                assert!(user.id < base_id);
                base_id = user.id;
            }
        }
    }

    #[sqlx::test(migrations = "../migrations", fixtures("users"))]
    async fn test_find_all_order_by_id_asc(pool: Pool<Postgres>) {
        let pagination = Pagination {
            offset: None,
            limit: None,
            order_by: Some(UserOrderFields::Id),
            order: Some(PaginationOrder::Asc),
        };
        let user_result = find_all(&pool, pagination).await;
        let users = user_result.unwrap();
        let user_length = users.len();

        assert_eq!(user_length, 10);
        let mut base_id: i32 = 0;
        for (index, user) in users.into_iter().enumerate() {
            if index == 0 {
                base_id = user.id;
            } else {
                assert!(user.id > base_id);
                base_id = user.id;
            }
        }
    }

    #[sqlx::test(migrations = "../migrations", fixtures("users"))]
    async fn test_offset(pool: Pool<Postgres>) {
        let pagination = Pagination {
            offset: None,
            limit: None,
            order_by: Some(UserOrderFields::Id),
            order: Some(PaginationOrder::Asc),
        };

        let mut user_result = find_all(&pool, pagination).await.unwrap();
        let last_user = user_result.pop().unwrap();

        let pagination = Pagination {
            offset: Some(10),
            limit: None,
            order_by: Some(UserOrderFields::Id),
            order: Some(PaginationOrder::Asc),
        };

        let user_result = find_all(&pool, pagination).await.unwrap();
        let user = &user_result[0];

        assert_eq!(user.id, last_user.id + 1)
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
