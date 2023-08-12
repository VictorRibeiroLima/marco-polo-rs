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

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_update_forgot_token_some_token(pool: PgPool) {
    let token = "123456";
    let id = 666;
    let tomorrow = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);

    update_forgot_token(&pool, id, Some(token)).await.unwrap();

    let user = find_by_id(&pool, id).await.unwrap();

    assert_eq!(user.forgot_token, Some(token.to_string()));
    assert_eq!(
        user.forgot_token_expires_at.unwrap().date(),
        tomorrow.date()
    );
}

#[sqlx::test(migrations = "../migrations", fixtures("forgotten_users"))]
async fn test_update_forgot_token_none_token(pool: PgPool) {
    let id = 6666;

    let token: Option<String> = None;

    update_forgot_token(&pool, id, token).await.unwrap();

    let user = find_by_id(&pool, id).await.unwrap();

    assert_eq!(user.forgot_token, None);
    assert_eq!(user.forgot_token_expires_at, None);
}

#[sqlx::test(migrations = "../migrations", fixtures("forgotten_users"))]
async fn test_update_password(pool: PgPool) {
    let password = "test";
    let id = 6666;

    update_password(&pool, id, password).await.unwrap();

    let user = find_by_id(&pool, id).await.unwrap();

    assert!(bcrypt::verify(password, &user.password).unwrap());

    assert_eq!(user.forgot_token, None);
    assert_eq!(user.forgot_token_expires_at, None);
}

#[sqlx::test(migrations = "../migrations", fixtures("forgotten_users"))]
async fn test_find_by_forgot_token_valid(pool: PgPool) {
    let token = "d1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d922";
    let user = find_by_forgot_token(&pool, token).await.unwrap();

    let user = user.unwrap();

    assert_eq!(user.id, 6666);
}

#[sqlx::test(migrations = "../migrations", fixtures("forgotten_users"))]
async fn test_find_by_forgot_token_almost_expired(pool: PgPool) {
    let token = "d1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d925";
    let user = find_by_forgot_token(&pool, token).await.unwrap();

    let user = user.unwrap();

    assert_eq!(user.id, 9999);
}

#[sqlx::test(migrations = "../migrations", fixtures("forgotten_users"))]
async fn test_find_by_forgot_token_expired_today(pool: PgPool) {
    let token = "d1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d923";
    let user = find_by_forgot_token(&pool, token).await.unwrap();

    assert!(user.is_none());
}

#[sqlx::test(migrations = "../migrations", fixtures("forgotten_users"))]
async fn test_find_by_forgot_token_expired_for_long_time(pool: PgPool) {
    let token = "d1596e0d4280f2bd2d311ce0819f23bde0dc834d8254b92924088de94c38d924";
    let user = find_by_forgot_token(&pool, token).await.unwrap();

    assert!(user.is_none());
}
