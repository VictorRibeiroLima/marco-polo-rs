use std::str::FromStr;
use std::sync::Arc;

use actix_http::Request;
use actix_web::{
    dev::ServiceResponse,
    http::header::ContentType,
    test,
    web::{self},
};
use marco_polo_rs_core::database::models::{user::UserRole, video::stage::VideoStage};
use reqwest::StatusCode;
use sqlx::PgPool;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    controllers::{
        test::{
            create_test_app,
            mock::{cloud_service::CloudServiceMock, video_platform::youtube::YoutubeClientMock},
        },
        video::dtos::VideoErrorDTO,
    },
    AppCloudService, AppYoutubeClient,
};

use crate::controllers::video::dtos::VideoDTO;

use crate::utils::test::get_token;
use crate::AppPool;

use super::create_scope;

#[cfg(test)]
mod create;

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/videos")
)]
async fn test_find_by_id_get_ok(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref(), 1000);

    let date = NaiveDate::from_ymd_opt(2022, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let expected_dto: VideoDTO = VideoDTO {
        id: uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap(),
        title: "Elon Musk Test".to_string(),
        description: "This is a test video about Elon Musk".to_string(),
        user_id: 456,
        original_url: "https://www.youtube.com/watch?v=1234567890".to_string(),
        channel_id: 666,
        url: Some("https://video.com".to_string()),
        language: "English".to_string(),
        created_at: date,
        updated_at: date,
        tags: Some(vec!["elon-musk".to_string(), "test".to_string()]),
        uploaded_at: Some(date),
        stage: VideoStage::Downloading,
        error: false,
        end_time: Some("00:05:00".to_string()),
        original_duration: Some("00:10:00".to_string()),
        start_time: "00:00:00".to_string(),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video/806b57d2-f221-11ed-a05b-0242ac120003")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: VideoDTO = test::read_body_json(response).await;
    assert_eq!(actual_dto, expected_dto);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/videos")
)]
async fn test_find_all_user(pool: PgPool) {
    let pool = Arc::new(pool);
    let user_id = 789;
    let token = get_token!(pool.as_ref(), user_id);

    let expected_videos_id: Vec<Uuid> = vec![
        "09a9e5f5-2c3b-4a54-bb1f-8a4d67c6156f",
        "2c20e6d2-7bce-47b7-b02d-7f45fb106df5",
        "48f6cbe7-4b88-45f1-8b7e-cac11dbf8f2e",
        "4e87a122-6f59-4a48-9ff6-6a5c9d7082e0",
    ]
    .iter()
    .map(|s| Uuid::from_str(s).unwrap())
    .collect();

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video?order=asc&order_by=id")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let actual_dto: Vec<VideoDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 4);

    let actual_videos_ids: Vec<Uuid> = actual_dto.iter().map(|video| video.id).collect();
    assert_eq!(actual_videos_ids, expected_videos_id);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/videos")
)]
async fn test_find_all_admin(pool: PgPool) {
    let pool = Arc::new(pool);
    let admin_id = 1000;

    let expected_videos_ids: Vec<Uuid> = vec![
        "05f06d54-0c32-485b-bde1-22bb8da09a5c",
        "07cc7053-6aee-4e27-9310-0e8593aee422",
        "09a9e5f5-2c3b-4a54-bb1f-8a4d67c6156f",
        "1c7b8db2-bd92-434b-9b4f-63d643a6f81d",
        "2c20e6d2-7bce-47b7-b02d-7f45fb106df5",
        "2c20e6d2-7bce-47b7-b02d-7f45fb106df7",
        "48f6cbe7-4b88-45f1-8b7e-cac11dbf8f2e",
        "4e87a122-6f59-4a48-9ff6-6a5c9d7082e0",
        "806b57d2-f221-11ed-a05b-0242ac120003",
        "9b594b49-c2b9-40a1-a20d-8d18a50dcd8d",
        "ac9a10b9-17e9-412f-a166-144a07a30e6d",
        "b7a720e3-010e-4d88-919b-7aee4d7a3144",
        "e4a399d1-7d97-432d-8681-30a6a94f88f5",
    ]
    .iter()
    .map(|s| Uuid::from_str(s).unwrap())
    .collect();

    let token = get_token!(pool.as_ref(), admin_id);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video?order_by=id&order=asc&limit=13")
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<VideoDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 13);
    let actual_videos_ids: Vec<Uuid> = actual_dto.iter().map(|video| video.id).collect();
    assert_eq!(actual_videos_ids, expected_videos_ids);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/videos")
)]
async fn test_find_all_5(pool: PgPool) {
    let pool = Arc::new(pool);
    let admin_id = 1000;
    let token = get_token!(pool.as_ref(), admin_id);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video?limit=5")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<VideoDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 5);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/videos")
)]
async fn test_find_by_id_get_not_found(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video/806b57d2-f221-11ed-a05b-0242ac120004")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/videos")
)]
async fn test_find_by_id_get_unauthorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video/806b57d2-f221-11ed-a05b-0242ac120003")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/user", "../../../test/fixtures/video")
)]
async fn test_find_by_id_get_deleted(pool: PgPool) {
    let pool = Arc::new(pool);
    let token = get_token!(pool.as_ref());
    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video/2c20e6d2-7bce-47b7-b02d-7f45fb106df5")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/admin", "../../../test/fixtures/videos")
)]
async fn test_find_all_admin_offset(pool: PgPool) {
    let pool = Arc::new(pool);
    let admin_id = 1000;

    let expected_videos_ids_first: Vec<Uuid> = vec![
        "05f06d54-0c32-485b-bde1-22bb8da09a5c",
        "07cc7053-6aee-4e27-9310-0e8593aee422",
        "09a9e5f5-2c3b-4a54-bb1f-8a4d67c6156f",
        "1c7b8db2-bd92-434b-9b4f-63d643a6f81d",
        "2c20e6d2-7bce-47b7-b02d-7f45fb106df5",
    ]
    .iter()
    .map(|s| Uuid::from_str(s).unwrap())
    .collect();

    let expected_videos_ids_second: Vec<Uuid> = vec![
        "48f6cbe7-4b88-45f1-8b7e-cac11dbf8f2e",
        "4e87a122-6f59-4a48-9ff6-6a5c9d7082e0",
        "806b57d2-f221-11ed-a05b-0242ac120003",
        "9b594b49-c2b9-40a1-a20d-8d18a50dcd8d",
        "ac9a10b9-17e9-412f-a166-144a07a30e6d",
        "b7a720e3-010e-4d88-919b-7aee4d7a3144",
    ]
    .iter()
    .map(|s| Uuid::from_str(s).unwrap())
    .collect();

    let token = get_token!(pool.as_ref(), admin_id);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/video?order_by=id&order=asc&limit=5&offset=0")
        .insert_header(("Authorization", token.clone()))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<VideoDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 5);
    let actual_videos_ids: Vec<Uuid> = actual_dto.iter().map(|video| video.id).collect();
    assert_eq!(actual_videos_ids, expected_videos_ids_first);

    let request = test::TestRequest::get()
        .uri("/video?order_by=id&order=asc&limit=6&offset=6")
        .insert_header(("Authorization", token))
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: Vec<VideoDTO> = test::read_body_json(response).await;
    assert_eq!(actual_dto.len(), 6);
    let actual_videos_ids: Vec<Uuid> = actual_dto.iter().map(|video| video.id).collect();
    assert_eq!(actual_videos_ids, expected_videos_ids_second);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/video", "../../../test/fixtures/admin")
)]
async fn test_find_video_errors_no_error(pool: PgPool) {
    let jwt = get_token!(&pool, 1000);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let video_id = "2c20e6d2-7bce-47b7-b02d-7f45fb106df5";

    let url = format!("/video/{}/errors", video_id);

    let request = test::TestRequest::get()
        .uri(&url)
        .insert_header(("Authorization", jwt))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let body: Vec<VideoErrorDTO> = test::read_body_json(response).await;

    assert_eq!(body.len(), 0);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/video_error", "../../../test/fixtures/admin")
)]
async fn test_find_video_errors_one_error(pool: PgPool) {
    let jwt = get_token!(&pool, 1000);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let video_id = "2c20e6d2-7bce-47b7-b02d-7f45fb106df5";

    let url = format!("/video/{}/errors", video_id);

    let request = test::TestRequest::get()
        .uri(&url)
        .insert_header(("Authorization", jwt))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let body: Vec<VideoErrorDTO> = test::read_body_json(response).await;

    assert_eq!(body.len(), 1);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/video_errors", "../../../test/fixtures/admin")
)]
async fn test_find_video_errors_three_error(pool: PgPool) {
    let jwt = get_token!(&pool, 1000);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let video_id = "2c20e6d2-7bce-47b7-b02d-7f45fb106df5";

    let url = format!("/video/{}/errors", video_id);

    let request = test::TestRequest::get()
        .uri(&url)
        .insert_header(("Authorization", jwt))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let body: Vec<VideoErrorDTO> = test::read_body_json(response).await;

    assert_eq!(body.len(), 3);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/video_errors", "../../../test/fixtures/user")
)]
async fn test_find_video_errors_three_errors_from_other_user(pool: PgPool) {
    let jwt = get_token!(&pool, 666);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let video_id = "2c20e6d2-7bce-47b7-b02d-7f45fb106df5";

    let url = format!("/video/{}/errors", video_id);

    let request = test::TestRequest::get()
        .uri(&url)
        .insert_header(("Authorization", jwt))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::OK);

    let body: Vec<VideoErrorDTO> = test::read_body_json(response).await;

    assert_eq!(body.len(), 0);
}

async fn innit_test_app(
    pool: Arc<PgPool>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };
    let web_data = web::Data::new(pool);
    let app_cloud_service = web::Data::new(AppCloudService {
        client: Arc::new(CloudServiceMock::new()),
    });
    let app_youtube_client = web::Data::new(AppYoutubeClient {
        client: Arc::new(YoutubeClientMock::new()),
    });

    let app = create_test_app();
    let scope = create_scope::<CloudServiceMock, YoutubeClientMock>();

    let app = app
        .app_data(web_data)
        .app_data(app_cloud_service)
        .app_data(app_youtube_client)
        .service(scope);

    let test_app = test::init_service(app).await;

    return test_app;
}
