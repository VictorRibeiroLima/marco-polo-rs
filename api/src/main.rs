use std::sync::Arc;

use actix_web::{
    get,
    web::{self, Json},
    App, HttpServer, Responder,
};
use marco_polo_rs_core::{database::create_pool, env};
use models::result::AppResult;

mod controllers;
mod middleware;
mod models;

struct GlobalState {
    pool: Arc<sqlx::PgPool>,
}

#[get("/")]
async fn hello() -> impl Responder {
    let result: AppResult<String> = AppResult::new("Hello, world!".to_string());
    return Json(result);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::check_envs();
    let pool = create_pool().await;
    let pool = Arc::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(GlobalState { pool: pool.clone() }))
            .service(hello)
            .configure(controllers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
