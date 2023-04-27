use dotenv;
use std::thread;

mod api;
mod internals;
mod queue;

fn check_envs() {
    //API
    std::env::var("API_URL").expect("API_URL not found");

    //AWS
    std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME not found");
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not found");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not found");
    std::env::var("AWS_QUEUE_URL").expect("AWS_QUEUE_URL not found");

    //ASSEMBLY_AI
    std::env::var("ASSEMBLY_AI_API_KEY").expect("ASSEMBLY_AI_API_KEY not found");
    std::env::var("ASSEMBLY_AI_BASE_URL").expect("ASSEMBLY_AI_BASE_URL not found");
    std::env::var("ASSEMBLY_AI_WEBHOOK_ENDPOINT").expect("ASSEMBLY_AI_WEBHOOK_ENDPOINT not found");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    check_envs();
    env_logger::init();
    thread::spawn(queue::init);
    api::init().await
}
