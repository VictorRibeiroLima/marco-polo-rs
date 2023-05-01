use std::sync::Arc;

use actix_web::rt::Runtime;

use crate::internals::{
    cloud::aws::AwsCloudService, subtitler::local::LocalClient,
    transcriber::assembly_ai::AssemblyAiClient, translator::deepl::DeeplClient,
};

use self::worker::Worker;

mod handlers;
mod worker;

pub fn init(pool: Arc<sqlx::PgPool>) {
    let rt = Runtime::new().unwrap();
    let queue_url = std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found");
    let cloud_service = AwsCloudService::new(queue_url).unwrap();
    let transcriber_client = AssemblyAiClient::new();
    let translator_client: DeeplClient = DeeplClient::new();
    let subtitler_client = LocalClient;

    let worker = Worker {
        pool,
        cloud_service,
        transcriber_client,
        translator_client,
        subtitler_client,
    };

    rt.block_on(async move {
        worker.handle_queue().await;
    });
}
