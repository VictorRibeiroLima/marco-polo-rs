use crate::api::models::result::AppResult;
use actix_web::{get, web::Json, App, HttpServer, Responder};

mod controllers;
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    let result: AppResult<String> = AppResult::new("Hello, world!".to_string());
    return Json(result);
}

pub async fn init() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .configure(controllers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
