use std::sync::Arc;

use actix_web::{
    get,
    web::{self, Json, JsonConfig},
    App, HttpServer, Responder,
};
use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::cloud::{default_cloud_service, traits::CloudService},
};
use models::{error::AppError, result::AppResult};

mod auth;
mod controllers;
mod middleware;
mod models;

struct AppPool {
    pool: Arc<sqlx::PgPool>,
}

struct AppCloudService<CS: CloudService> {
    client: Arc<CS>,
}

#[get("/")]
async fn hello() -> impl Responder {
    let result: AppResult<String> = AppResult::new("Hello, world!".to_string());
    return Json(result);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
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
            .app_data(web::Data::new(AppPool { pool: pool.clone() }))
            .app_data(web::Data::new(AppCloudService {
                client: cloud_service.clone(),
            }))
            .service(hello)
            .configure(controllers::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
