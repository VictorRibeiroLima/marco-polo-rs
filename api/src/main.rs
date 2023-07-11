use std::sync::Arc;

use actix_web::{
    get,
    web::{self, Json, JsonConfig, QueryConfig},
    App, HttpServer, Responder,
};
use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::{
        cloud::{default_cloud_service, traits::CloudService},
        youtube_client::{client, traits},
    },
};
use models::{error::AppError, result::AppResult};

mod auth;
mod controllers;
mod middleware;
mod models;
mod utils;

struct AppPool {
    pool: Arc<sqlx::PgPool>,
}

struct AppYoutubeClient {
    client: Arc<dyn traits::YoutubeClient>,
}

struct AppCloudService<CS: CloudService> {
    client: Arc<CS>,
}

#[get("/")]
async fn hello() -> impl Responder {
    //coment
    let result: AppResult<String> = AppResult::new("I'm alive".to_string());
    return Json(result);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    let youtube_client = client::YoutubeClient::new();
    let youtube_client = Arc::new(youtube_client);
    dotenv::dotenv().ok();
    env::check_envs();
    let pool = create_pool().await;
    let pool = Arc::new(pool);

    let cloud_service = default_cloud_service();
    let cloud_service = Arc::new(cloud_service);

    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                let error = AppError::from(err);
                return error.into();
            }))
            .app_data(QueryConfig::default().error_handler(|err, _req| {
                let error = AppError::from(err);
                return error.into();
            }))
            .app_data(web::Data::new(AppPool { pool: pool.clone() }))
            .app_data(web::Data::new(AppCloudService {
                client: cloud_service.clone(),
            }))
            .app_data(web::Data::new(AppYoutubeClient {
                client: youtube_client.clone(),
            }))
            .service(hello)
            .configure(controllers::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
