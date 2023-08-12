use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    get,
    web::{self, Json, JsonConfig, QueryConfig},
    App, HttpServer, Responder,
};
use mail::{engine::MailEngine, sender::MailSender, Mailer};
use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::{
        cloud::{default_cloud_service, traits::CloudService},
        youtube_client::{client, traits::YoutubeClient},
    },
};
use models::{error::AppError, result::AppResult};

mod auth;
mod controllers;
mod mail;
mod middleware;
mod models;
mod utils;

struct AppPool {
    pool: Arc<sqlx::PgPool>,
}

struct AppYoutubeClient<YC: YoutubeClient> {
    client: Arc<YC>,
}

struct AppCloudService<CS: CloudService> {
    client: Arc<CS>,
}

struct AppMailer<E: MailEngine, S: MailSender> {
    mailer: Arc<Mailer<E, S>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    let result: AppResult<String> = AppResult::new("I'm alive 7".to_string());
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

    let app_mailer = Arc::new(mail::Mailer::default());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
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
            .app_data(web::Data::new(AppMailer {
                mailer: app_mailer.clone(),
            }))
            .service(hello)
            .configure(controllers::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
