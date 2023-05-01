use dotenv;
use internals::cloud::traits::BucketClient;
use std::{sync::Arc, thread};

mod api;
mod database;
mod internals;
mod queue;
mod util;

fn check_envs() {
    //DATABASE
    std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    //API
    std::env::var("API_URL").expect("API_URL not found");
    std::env::var("API_KEY").expect("API_KEY not found");

    //AWS
    std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME not found");
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not found");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not found");
    std::env::var("AWS_QUEUE_URL").expect("AWS_QUEUE_URL not found");

    //ASSEMBLY_AI
    std::env::var("ASSEMBLY_AI_API_KEY").expect("ASSEMBLY_AI_API_KEY not found");
    std::env::var("ASSEMBLY_AI_BASE_URL").expect("ASSEMBLY_AI_BASE_URL not found");
    std::env::var("ASSEMBLY_AI_WEBHOOK_ENDPOINT").expect("ASSEMBLY_AI_WEBHOOK_ENDPOINT not found");
    std::env::var("ASSEMBLY_AI_WEBHOOK_TOKEN").expect("ASSEMBLY_AI_WEBHOOK_TOKEN not found");

    //DEEPL
    std::env::var("DEEPL_BASE_URL").expect("DEEPL_BASE_URL not set");
    std::env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    check_envs();

    /*
    let bucket_client = internals::cloud::aws::s3::S3Client::new().unwrap();

    let root = std::env::current_dir().unwrap();
    let temp_dir = root.join("temp");
    let video_id = "839b6b64-ace9-4425-b017-717aca77ea3c";
    let output_path = temp_dir.join(format!("output_{}.{}", video_id, "mp4"));
    let output_bytes = std::fs::read(&output_path)?;
    let output_uri = format!("videos/processed/{}.{}", video_id, "mp4");

    println!("output_path: {:?}", output_path);
    bucket_client
    .upload_file(&output_uri, output_bytes)
    .await
    .unwrap();*/

    let pool = Arc::new(database::create_pool().await);
    let thread_pool = pool.clone();

    env_logger::init();

    thread::spawn(move || {
        queue::init(thread_pool);
    });

    api::init(pool).await
}
