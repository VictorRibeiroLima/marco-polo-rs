use std::sync::Arc;

use actix_http::StatusCode;
use marco_polo_rs_core::database::models::video::Video;
use sqlx::PgPool;

use actix_web::{
    http::header::ContentType,
    test,
    web::{self, post},
    App,
};

use crate::{
    controllers::{
        test::mock::{cloud_service::CloudServiceMock, youtube_client::YoutubeClientMock},
        video::dtos::{
            create::{Create, Cut},
            VideoDTO,
        },
    },
    models::error::AppErrorResponse,
    utils::test::get_token,
    AppCloudService, AppPool, AppYoutubeClient,
};

use super::{super::create_video, innit_test_app};

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_video_ok(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 1,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let video: Video = sqlx::query_as("SELECT * FROM videos WHERE channel_id = 1")
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

    assert_eq!(video.title, dto.cuts[0].title);
    assert_eq!(video.description, dto.cuts[0].description);
    assert_eq!(video.channel_id, dto.cuts[0].channel_id);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels", "../../../test/fixtures/admin")
)]
async fn test_create_video_ok_admin(pool: PgPool) {
    let jwt = get_token!(&pool, 1000);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 1,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        start_time: Some("00:00:00".to_string()),
        end_time: Some("00:10:00".to_string()),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let video: Video = sqlx::query_as("SELECT * FROM videos WHERE channel_id = 1")
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

    assert_eq!(video.title, dto.cuts[0].title);
    assert_eq!(video.description, dto.cuts[0].description);
    assert_eq!(video.channel_id, dto.cuts[0].channel_id);
    assert_eq!(
        video.start_time,
        dto.cuts[0].start_time.as_ref().unwrap().to_string()
    );
    assert_eq!(video.end_time, dto.cuts[0].end_time);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_video_not_found_when_channel_does_not_belong(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 2,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_video_channel_has_error(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 46,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);

    let body: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(
        body.errors[0],
        "Channel has errors. Please contact admins".to_string()
    );
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_video_channel_does_not_has_refresh_token(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 52,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);

    let body: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(body.errors[0], "Youtube channel not linked".to_string());
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_video_channel_error_when_getting_info(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);

    let pool = AppPool { pool };

    let web_data = web::Data::new(pool);
    let app_cloud_service = web::Data::new(AppCloudService {
        client: Arc::new(CloudServiceMock::new()),
    });
    let app_youtube_client = web::Data::new(AppYoutubeClient {
        client: Arc::new(YoutubeClientMock::with_error()),
    });
    let app = App::new()
        .app_data(web_data)
        .app_data(app_cloud_service)
        .app_data(app_youtube_client)
        .route(
            "/",
            post().to(create_video::<CloudServiceMock, YoutubeClientMock>),
        );

    let test_app = test::init_service(app).await;

    let cut = Cut {
        channel_id: 1,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);

    let body: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(
        body.errors[0],
        "Channel has errors. Please contact admins".to_string()
    );
}
#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_returning_id(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 1,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let actual_dto: Vec<VideoDTO> = test::read_body_json(response).await;

    assert_eq!(actual_dto.len(), 1);

    let video: Video = sqlx::query_as("SELECT * FROM videos WHERE channel_id = 1")
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

    assert_eq!(actual_dto[0].id, video.id);
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_video_bad_request_start_time_end_time(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut = Cut {
        channel_id: 1,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        start_time: Some("test:00".to_string()),
        end_time: Some("test:00".to_string()),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    let body: AppErrorResponse = test::read_body_json(response).await;

    assert_eq!(body.errors.len(), 2);

    for error in body.errors {
        let error = error.split(": ").collect::<Vec<&str>>().pop().unwrap();
        assert_eq!(error, "Invalid Time Format (HH:MM:SS)".to_string());
    }
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures("../../../test/fixtures/channels")
)]
async fn test_create_multiple_cuts(pool: PgPool) {
    let jwt = get_token!(&pool, 1);
    let pool = Arc::new(pool);
    let app = innit_test_app(pool.clone()).await;

    let cut1 = Cut {
        channel_id: 1,
        description: "This is a test video about Elon Musk".to_string(),
        title: "Elon Musk Test".to_string(),
        end_time: Some("00:03:00".to_string()),
        ..Default::default()
    };

    let cut2 = Cut {
        channel_id: 4,
        description: "This is a test video about Elon Musk 2".to_string(),
        title: "Elon Musk Test 2".to_string(),
        start_time: Some("00:03:00".to_string()),
        end_time: Some("00:06:00".to_string()),
        ..Default::default()
    };

    let cut3 = Cut {
        channel_id: 7,
        description: "This is a test video about Elon Musk 3".to_string(),
        title: "Elon Musk Test 3".to_string(),
        start_time: Some("00:06:00".to_string()),
        ..Default::default()
    };

    let dto = Create {
        video_url: "https://www.youtube.com/watch?v=1".to_string(),
        cuts: vec![cut1, cut2, cut3],
        ..Default::default()
    };

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(("Authorization", jwt))
        .insert_header(ContentType::json())
        .set_json(&dto)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::CREATED);
}
