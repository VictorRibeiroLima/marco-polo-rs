use std::sync::{Arc, Mutex};

use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::{
        cloud::aws::AwsCloudService, subtitler::videobox::VideoBoxClient,
        transcriber::assembly_ai::AssemblyAiClient, translator::deepl::DeeplClient,
    },
    util::queue::Queue,
};
use worker::Worker;

mod handlers;
mod srt;
mod worker;

#[tokio::main]
async fn main() {
    println!("Starting worker...");
    dotenv::dotenv().ok();
    env::check_envs();
    let pool = create_pool().await;
    let pool = Arc::new(pool);

    let queue_url = std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found");
    let cloud_service = AwsCloudService::new(queue_url).unwrap();
    let cloud_service = Arc::new(cloud_service);

    let transcriber_client = AssemblyAiClient::new();
    let transcriber_client = Arc::new(transcriber_client);

    let translator_client: DeeplClient = DeeplClient::new();
    let translator_client = Arc::new(translator_client);

    let subtitler_client = VideoBoxClient::new();
    let subtitler_client = Arc::new(subtitler_client);

    let message_pool = Queue::new();
    let message_pool = Arc::new(Mutex::new(message_pool));

    let worker = Worker {
        pool,
        cloud_service,
        transcriber_client,
        translator_client,
        subtitler_client,
        message_pool,
    };

    worker.handle_queue().await;
}
