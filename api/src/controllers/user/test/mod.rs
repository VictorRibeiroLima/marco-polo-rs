use std::sync::Arc;

use actix_http::Request;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web, App};
use marco_polo_rs_core::database::models::user::UserRole;
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::controllers::user::dtos::create::CreateUser;

use crate::{controllers::user::create_user, AppPool};

#[sqlx::test(migrations = "../migrations")]
async fn test_create_user_valid_email_and_password(pool: PgPool) {
    let pool = Arc::new(pool);

    let create_user_dto = CreateUser {
        name: "Test".to_string(),
        email: "test1233@gmail.com".to_string(),
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

async fn innit_test_app(
    pool: Arc<PgPool>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };
    let web_data = web::Data::new(pool);
    let app = App::new().app_data(web_data).service(create_user);

    let test_app = test::init_service(app).await;

    return test_app;
}
