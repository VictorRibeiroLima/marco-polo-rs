use std::str::FromStr;
use std::sync::Arc;

use actix_http::Request;
use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web, App};
use marco_polo_rs_core::database::models::user::{User, UserRole};
use reqwest::StatusCode;
use sqlx::PgPool;

use chrono::NaiveDate;

use crate::auth::gen_token;

use crate::controllers::video::dtos::create::VideoDTO;

use crate::utils::test::get_token;
use crate::AppPool;

use super::find_by_id;

#[sqlx::test(migrations = "../migrations", fixtures("user", "videos"))]
async fn test_find_by_id_get_ok(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let date = NaiveDate::from_ymd_opt(2022, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let expected_dto: VideoDTO = VideoDTO {
        id: uuid::Uuid::from_str("806b57d2-f221-11ed-a05b-0242ac120003").unwrap(),
        title: "Elon Musk Test".to_string(),
        description: "This is a test video about Elon Musk".to_string(),
        user_id: 123,
        channel_id: 666,
        url: Some("https://video.com".to_string()),
        language: "English".to_string(),
        created_at: date,
        updated_at: date,
        uploaded_at: Some(date),
    };

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/806b57d2-f221-11ed-a05b-0242ac120003")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::OK);
    let actual_dto: VideoDTO = test::read_body_json(response).await;
    assert_eq!(actual_dto, expected_dto);
}

#[sqlx::test(migrations = "../migrations", fixtures("user", "videos"))]
async fn test_find_by_id_get_not_found(pool: PgPool) {
    let pool = Arc::new(pool);

    let token = get_token!(pool.as_ref());

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/806b57d2-f221-11ed-a05b-0242ac120004")
        .insert_header(ContentType::json())
        .insert_header(("Authorization", token))
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../migrations", fixtures("user", "videos"))]
async fn test_find_by_id_get_unauthorized(pool: PgPool) {
    let pool = Arc::new(pool);

    let test_app = innit_test_app(pool.clone()).await;

    let request = test::TestRequest::get()
        .uri("/806b57d2-f221-11ed-a05b-0242ac120003")
        .insert_header(ContentType::json())
        .to_request();

    let response = test::call_service(&test_app, request).await;
    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}

async fn innit_test_app(
    pool: Arc<PgPool>,
) -> impl actix_web::dev::Service<Request, Response = ServiceResponse, Error = actix_web::Error> {
    let pool = AppPool { pool };
    let web_data = web::Data::new(pool);
    let app = App::new().app_data(web_data).service(find_by_id);

    let test_app = test::init_service(app).await;

    return test_app;
}
