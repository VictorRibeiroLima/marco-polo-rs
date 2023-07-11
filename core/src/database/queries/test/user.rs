use core::panic;

use sqlx::{PgPool, Pool, Postgres};

use crate::database::{
    models::user::{UserOrderFields, UserRole},
    queries::user::*,
};

use super::macros::test_find_all;

test_find_all!(User, UserOrderFields, find_all, "users");

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
