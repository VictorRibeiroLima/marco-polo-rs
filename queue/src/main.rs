use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::{
        cloud::aws::AwsCloudService, subtitler::local::LocalClient,
        transcriber::assembly_ai::AssemblyAiClient, translator::deepl::DeeplClient,
    },
};
use worker::Worker;

mod handlers;
mod srt;
mod worker;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env::check_envs();
    let pool = create_pool().await;
    let queue_url = std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found");
    let cloud_service = AwsCloudService::new(queue_url).unwrap();
    let transcriber_client = AssemblyAiClient::new();
    let translator_client: DeeplClient = DeeplClient::new();
    let subtitler_client = LocalClient::new();

    let worker = Worker {
        pool,
        cloud_service,
        transcriber_client,
        translator_client,
        subtitler_client,
    };

    worker.handle_queue().await;
}
