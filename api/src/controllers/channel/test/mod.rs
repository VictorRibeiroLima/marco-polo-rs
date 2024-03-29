use std::sync::Arc;

use actix_http::Request;
use actix_web::{
    dev::ServiceResponse,
    http::header::ContentType,
    test,
    web::{self},
};
use chrono::NaiveDate;
use marco_polo_rs_core::database::models::{
    channel::{auth::AuthType, platform::Platform, Channel},
    user::UserRole,
};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::{
    controllers::{
        channel::dto::ChannelDTO,
        test::{
            create_test_app,
            mock::video_platform::youtube::{YoutubeClientMock, CSRF_TOKEN},
        },
    },
    utils::test::get_token,
    AppPool, AppYoutubeClient,
};

use super::create_scope;

#[sqlx::test(migrations = "../migrations")]
async fn test_create_channel_unauthorized(pool: PgPool) {
    let test_app = innit_test_app(Arc::new(pool)).await;

    let request = test::TestRequest::post()
        .uri("/channel/youtube")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(migrations = "../migrations", fixtures("../../../test/fixtures/user"))]
async fn test_create_channel_authorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::post()
        .uri("/channel/youtube")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let record = sqlx::query!(
        r#"
        SELECT COUNT(*) FROM channels WHERE auth -> 'data' ->> 'csrf_token' = $1
        "#,
        CSRF_TOKEN
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    assert!(record.count.is_some());

    assert_eq!(record.count.unwrap(), 1);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/channel")
)]
async fn test_find_by_id_get_deleted(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/2")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/channel")
)]
async fn test_find_by_id_get_ok(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let date = NaiveDate::from_ymd_opt(2022, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let expected_dto: ChannelDTO = ChannelDTO {
        id: 1,
        name: Some("Test Channel".to_string()),
        creator_id: 999,
        created_at: date,
        updated_at: date,
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/channel/1")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: ChannelDTO = test::read_body_json(response).await;
    assert_eq!(actual_dto, expected_dto);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/channel")
)]
async fn test_find_by_id_get_not_found(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/3")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/channel")
)]
async fn test_find_by_id_get_unauthorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/channel/1")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/channels")
)]
async fn test_find_all_admin(pool: PgPool) {
    let pool = Arc::new(pool);
    let admin_id = 1000;

    let mut expected_channels_ids = vec![];
    for i in 1..=45 {
        expected_channels_ids.push(i);
    }

    let token = get_token!(pool.as_ref(), admin_id);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/channel?order_by=id&order=asc&limit=45")
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<ChannelDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 45);
    let actual_channels_ids: Vec<i32> = actual_dto.iter().map(|channel| channel.id).collect();
    assert_eq!(actual_channels_ids, expected_channels_ids);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/channels")
)]
async fn test_find_all_admin_offset(pool: PgPool) {
    let pool = Arc::new(pool);
    let admin_id = 1000;

    let mut expected_channels_ids_first = vec![];
    for i in 1..=20 {
        expected_channels_ids_first.push(i);
    }

    let mut expected_channels_ids_second = vec![];
    for i in 21..=40 {
        expected_channels_ids_second.push(i);
    }

    let token = get_token!(pool.as_ref(), admin_id);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/channel?order_by=id&order=asc&limit=20&offset=0")
        .insert_header(("Authorization", token.clone()))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<ChannelDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 20);
    let actual_channels_ids: Vec<i32> = actual_dto.iter().map(|channel| channel.id).collect();
    assert_eq!(actual_channels_ids, expected_channels_ids_first);

    let request = test::TestRequest::get()
        .uri("/channel?order_by=id&order=asc&limit=20&offset=20")
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<ChannelDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 20);
    let actual_channels_ids: Vec<i32> = actual_dto.iter().map(|channel| channel.id).collect();
    assert_eq!(actual_channels_ids, expected_channels_ids_second);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_find_all_user(pool: PgPool) {
    let pool = Arc::new(pool);
    let user_id = 1;

    let expected_channels_ids = vec![1, 4, 7, 10, 13, 16, 19, 22, 25, 28];

    let token = get_token!(pool.as_ref(), user_id);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/channel?order=asc&order_by=id")
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<ChannelDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 10);

    let actual_channels_ids: Vec<i32> = actual_dto.iter().map(|channel| channel.id).collect();
    assert_eq!(actual_channels_ids, expected_channels_ids);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/channels")
)]
async fn admin_can_resign_channel(pool: PgPool) {
    let admin_id = 1000;
    let channel_id = 1;

    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref(), admin_id);

    let test_app = innit_test_app(pool.clone()).await;

    let uri = format!("/channel/youtube/resign/{}", channel_id);

    let request = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let channel: Channel = sqlx::query_as!(
        Channel,
        r#"SELECT  
                id,
                name,
                creator_id,
                error,
                platform as "platform: Platform",
                auth as "auth: sqlx::types::Json<AuthType>",
                created_at as "created_at: chrono::NaiveDateTime",
                updated_at as "updated_at: chrono::NaiveDateTime",
                deleted_at as "deleted_at: chrono::NaiveDateTime" 
            FROM channels WHERE id = $1"#,
        channel_id
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    let auth = match channel.auth.0 {
        AuthType::Oauth2(auth) => auth,
        _ => panic!("Auth type is not oauth2"),
    };

    assert!(auth.refresh_token.is_none());
    assert_eq!(auth.csrf_token.unwrap(), CSRF_TOKEN);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn user_can_resign_on_channel(pool: PgPool) {
    let user_id = 1;
    let channel_id = 1;

    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref(), user_id);

    let test_app = innit_test_app(pool.clone()).await;

    let uri = format!("/channel/youtube/resign/{}", channel_id);

    let request = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let channel: Channel = sqlx::query_as!(
        Channel,
        r#"SELECT  
                id,
                name,
                creator_id,
                error,
                platform as "platform: Platform",
                auth as "auth: sqlx::types::Json<AuthType>",
                created_at as "created_at: chrono::NaiveDateTime",
                updated_at as "updated_at: chrono::NaiveDateTime",
                deleted_at as "deleted_at: chrono::NaiveDateTime" 
            FROM channels WHERE id = $1"#,
        channel_id
    )
    .fetch_one(pool.as_ref())
    .await
    .unwrap();

    let auth = match channel.auth.0 {
        AuthType::Oauth2(auth) => auth,
        _ => panic!("Auth type is not oauth2"),
    };

    assert!(auth.refresh_token.is_none());
    assert_eq!(auth.csrf_token.unwrap(), CSRF_TOKEN);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn user_can_not_resign_on_channel_if_not_owner(pool: PgPool) {
    let user_id = 2;
    let channel_id = 1;

    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref(), user_id);

    let test_app = innit_test_app(pool.clone()).await;

    let uri = format!("/channel/youtube/resign/{}", channel_id);

    let request = test::TestRequest::put()
        .uri(&uri)
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

async fn innit_test_app(
    pool: Arc<PgPool>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };

    let youtube_client = YoutubeClientMock::new();
    let youtube_client = Arc::new(youtube_client);

    let youtube_client = AppYoutubeClient {
        client: youtube_client,
    };
    let web_data = web::Data::new(pool);

    let app = create_test_app();

    let channel_scope = create_scope::<YoutubeClientMock>();

    let app = app
        .app_data(web_data)
        .app_data(web::Data::new(youtube_client))
        .service(channel_scope);

    let test_app = test::init_service(app).await;

    return test_app;
}
