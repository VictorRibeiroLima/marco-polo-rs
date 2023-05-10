use std::sync::Arc;

use actix_web::{http::header::ContentType, test, web, App};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::{
    controllers::channel::{create_channel, dtos::CreateChannel},
    GlobalState,
};

#[sqlx::test(migrations = "../migrations")]
async fn test_create_channel(pool: PgPool) {
    let pool = Arc::new(pool);
    let global_state = GlobalState { pool };
    let web_data = web::Data::new(global_state);
    let app = App::new().app_data(web_data).service(create_channel);

    let create_channel_dto = CreateChannel {
        name: String::from("ElonMusk Cortes"),
    };

    let test_app = test::init_service(app).await;

    let request = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(create_channel_dto)
        .to_request();

    let response = test::call_service(&test_app, request).await;

    assert_eq!(response.status().as_u16(), StatusCode::UNAUTHORIZED);
}
