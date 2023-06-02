use std::sync::Arc;

use actix_http::Request;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web, App};
use chrono::{DateTime, Utc};
use marco_polo_rs_core::{
    database::models::user::{User, UserRole},
    internals::youtube_client::traits,
    SyncError,
};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::{
    auth::gen_token, controllers::channel::create_youtube_channel, AppPool, AppYoutubeClient,
};

const CSRF_TOKEN: &str = "111aaa11aa";

struct YoutubeClientMock;

#[async_trait::async_trait]
impl traits::YoutubeClient for YoutubeClientMock {
    fn generate_url(&self) -> (String, String) {
        return (
            String::from("https://youtube.com"),
            String::from(CSRF_TOKEN),
        );
    }

    async fn get_refresh_token(&self, _code: String) -> Result<String, SyncError> {
        return Ok(String::from("refresh_token"));
    }

    async fn get_channel_info(&self, _refresh_token: String) -> Result<String, SyncError> {
        return Ok(String::from("channel_info"));
    }
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_channel_unauthorized(pool: PgPool) {
    let youtube_client = YoutubeClientMock;
    let youtube_client = Arc::new(youtube_client);
    let test_app = innit_test_app(Arc::new(pool), youtube_client).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_create_channel_authorized(pool: PgPool) {
    std::env::set_var("API_JSON_WEB_TOKEN_SECRET", "test_secret");
    let pool = Arc::new(pool);

    let youtube_client = YoutubeClientMock;
    let youtube_client = Arc::new(youtube_client);

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

    let test_app = innit_test_app(pool.clone(), youtube_client).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let record = sqlx::query!(
        r#"
        SELECT COUNT(*) FROM channels WHERE csrf_token = $1
        "#,
        CSRF_TOKEN
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(record.count.is_some());

    assert_eq!(record.count.unwrap(), 1);
}

async fn innit_test_app(
    pool: Arc<PgPool>,
    youtube_client: Arc<YoutubeClientMock>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };
    let youtube_client = AppYoutubeClient {
        client: youtube_client,
    };
    let web_data = web::Data::new(pool);
    let app = App::new()
        .app_data(web_data)
        .app_data(web::Data::new(youtube_client))
        .service(create_youtube_channel);

    let test_app = test::init_service(app).await;

    return test_app;
}
