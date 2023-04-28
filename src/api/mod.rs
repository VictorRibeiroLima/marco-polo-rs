use std::sync::Arc;

use crate::api::models::result::AppResult;
use actix_web::{
    get,
    web::{self, Json},
    App, HttpServer, Responder,
};

mod controllers;
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    let result: AppResult<String> = AppResult::new("Hello, world!".to_string());
    return Json(result);
}

struct GlobalState {
    pool: Arc<sqlx::PgPool>,
}

pub async fn init(pool: sqlx::PgPool) -> std::io::Result<()> {
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
