use google_youtube3::api::Video;
use marco_polo_rs_core::database::models::video::VideoWithStorageAndChannel;
use std::sync::Arc;

use actix_http::Request;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web, App};
use chrono::{DateTime, Utc};
use marco_polo_rs_core::{
    database::models::user::{User, UserRole},
    internals::youtube_client::{channel_info::ChannelInfo, traits},
    SyncError,
};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::{
    auth::gen_token, controllers::channel::create_youtube_channel, AppPool, AppYoutubeClient, utils::test::get_token,
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

    async fn get_channel_info(&self, _refresh_token: String) -> Result<ChannelInfo, SyncError> {
        return Ok(ChannelInfo::default());
    }

    async fn upload_video(&self, _: &VideoWithStorageAndChannel) -> Result<Video, SyncError> {
        return Ok(Default::default());
    }
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_channel_unauthorized(pool: PgPool) {
    let youtube_client = YoutubeClientMock;
    let youtube_client = Arc::new(youtube_client);
    let test_app = innit_test_app(Arc::new(pool), youtube_client).await;

    let request = test::TestRequest::post()
        .uri("/youtube")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_create_channel_authorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let youtube_client = YoutubeClientMock;
    let youtube_client = Arc::new(youtube_client);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone(), youtube_client).await;

    let request = test::TestRequest::post()
        .uri("/youtube")
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

#[sqlx::test(migrations = "../migrations", fixtures("user"))]
async fn test_find_by_id_get_ok(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());
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
