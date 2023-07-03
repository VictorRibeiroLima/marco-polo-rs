use std::sync::Arc;

use actix_http::Request;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web, App};
use marco_polo_rs_core::database::models::user::{User, UserRole};
use reqwest::StatusCode;
use sqlx::PgPool;

use chrono::{DateTime, Utc};

use crate::auth::gen_token;
use crate::controllers::user::dtos::create::CreateUser;

use crate::controllers::user::dtos::find::UserDTO;
use crate::{controllers::user::create_user, AppPool};

use super::{find_by_id, login};

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_valid_email_and_password(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "test123@gmail.com".to_string(),
        password: "12345aA!".to_string(),
        role: Some(UserRole::User),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(&create_user_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let record = sqlx::query!(
        r#"
        SELECT COUNT(*) FROM users WHERE name = $1
        "#,
        "Test"
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(record.count.is_some());

    assert_eq!(record.count.unwrap(), 1);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_invalid_email(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "Not_a_email".to_string(),
        password: "12345aA!".to_string(),
        role: Some(UserRole::User),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(&create_user_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_invalid_password(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "test123@gmail.com".to_string(),
        password: "123".to_string(),
        role: Some(UserRole::User),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(&create_user_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_find_by_id_get_unauthorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/666")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_find_by_id_get_not_found(pool: PgPool) {
    std::env::set_var("API_JSON_WEB_TOKEN_SECRET", "test_secret");
    let pool = Arc::new(pool);

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, 
        name, 
        email, 
        password, 
        role as "role: UserRole",
        created_at as "created_at: DateTime <Utc>",
        updated_at as "updated_at: DateTime <Utc>",
        deleted_at as "deleted_at: DateTime <Utc>"
        FROM users WHERE id = 666"#
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    let token = gen_token(user).await.unwrap();

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/665")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_find_by_id_get_deleted(pool: PgPool) {
    std::env::set_var("API_JSON_WEB_TOKEN_SECRET", "test_secret");
    let pool = Arc::new(pool);

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, 
        name, 
        email, 
        password, 
        role as "role: UserRole",
        created_at as "created_at: DateTime <Utc>",
        updated_at as "updated_at: DateTime <Utc>",
        deleted_at as "deleted_at: DateTime <Utc>"
        FROM users WHERE id = 666"#
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    let token = gen_token(user).await.unwrap();

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/667")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_find_by_id_get_ok(pool: PgPool) {
    std::env::set_var("API_JSON_WEB_TOKEN_SECRET", "test_secret");
    let pool = Arc::new(pool);

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, 
        name, 
        email, 
        password, 
        role as "role: UserRole",
        created_at as "created_at: DateTime <Utc>",
        updated_at as "updated_at: DateTime <Utc>",
        deleted_at as "deleted_at: DateTime <Utc>"
        FROM users WHERE id = 666"#
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    let token = gen_token(user.clone()).await.unwrap();
    let expected_dto: UserDTO = user.into();

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/666")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: UserDTO = test::read_body_json(response).await;
    assert_eq!(actual_dto, expected_dto);
}

async fn innit_test_app(
    pool: Arc<PgPool>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };
    let web_data = web::Data::new(pool);
    let app = App::new()
        .app_data(web_data)
        .service(create_user)
        .service(login)
        .service(find_by_id);

    let test_app = test::init_service(app).await;

    return test_app;
}
