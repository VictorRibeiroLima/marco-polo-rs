use std::sync::Arc;

use actix_http::Request;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web, App};
use chrono::{DateTime, Utc};
use marco_polo_rs_core::database::models::user::{User, UserRole};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::{
    auth::gen_token,
    controllers::channel::{create_channel, dtos::CreateChannel},
    GlobalState,
};

#[sqlx::test(migrations = "../migrations")]
async fn test_create_channel_unauthorized(pool: PgPool) {
    let create_channel_dto = CreateChannel {
        name: String::from("ElonMusk Cortes"),
    };

    let test_app = innit_test_app(Arc::new(pool)).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(create_channel_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_create_channel_authorized(pool: PgPool) {
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

    let create_channel_dto = CreateChannel {
        name: String::from("ElonMusk Cortes"),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .set_json(create_channel_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let record = sqlx::query!(
        r#"
        SELECT COUNT(*) FROM channels WHERE name = 'ElonMusk Cortes'
    "#
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
    let global_state = GlobalState { pool };
    let web_data = web::Data::new(global_state);
    let app = App::new().app_data(web_data).service(create_channel);

    let test_app = test::init_service(app).await;

    return test_app;
}
