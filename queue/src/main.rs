use std::sync::Arc;

use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::{
        cloud::{
            aws::AwsCloudService,
            traits::{CloudService, QueueClient},
        },
        subtitler::local::LocalClient,
        transcriber::assembly_ai::AssemblyAiClient,
        translator::deepl::DeeplClient,
    },
    util::queue::Queue,
};
use sqlx::PgPool;
use tokio::{
    runtime::{Builder, Runtime},
    sync::Mutex,
    task::JoinHandle,
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

    let message_pool = Queue::new();
    let message_pool = Arc::new(Mutex::new(message_pool));

    let runtime = Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
        .unwrap();

    spawn_workers(pool, &runtime, cloud_service.clone(), message_pool.clone());

    let queue_client = cloud_service.queue_client();
    loop {
        let message_result = queue_client.receive_message().await.unwrap();
        let messages = match message_result {
            Some(messages) => messages,
            _ => {
                continue;
            }
        };
        let mut lock = message_pool.lock().await;
        println!("Enqueuing {} messages", messages.len());
        for message in messages {
            lock.enqueue(message);
        }
        drop(lock);
    }
}

fn spawn_workers<CS>(
    pool: Arc<PgPool>,
    runtime: &Runtime,
    cloud_service: CS,
    message_pool: Arc<Mutex<Queue<<<CS as CloudService>::QC as QueueClient>::M>>>,
) -> Vec<JoinHandle<()>>
where
    CS: 'static + CloudService + Clone + Sync + Send,
    <CS as CloudService>::QC: Sync + Send,
    <<CS as CloudService>::QC as QueueClient>::M: Send + Sync,
{
    let handles: Vec<JoinHandle<()>> = (0..num_cpus::get())
        .map(|id| {
            let transcriber_client = AssemblyAiClient::new();
            let translator_client: DeeplClient = DeeplClient::new();
            let subtitler_client = LocalClient::new();

            let worker = Worker {
                id,
                pool: pool.clone(),
                cloud_service: cloud_service.clone(),
                transcriber_client,
                translator_client,
                subtitler_client,
                message_pool: message_pool.clone(),
            };

            runtime.spawn(async move {
                worker.handle_queue().await;
            })
        })
        .collect();

    return handles;
}
