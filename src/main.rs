use dotenv;
use std::thread;

mod api;
mod internals;
mod queue;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    env_logger::init();
    thread::spawn(queue::init);
    api::init().await
}
