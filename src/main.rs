mod api;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    env_logger::init();
    api::init().await
}
