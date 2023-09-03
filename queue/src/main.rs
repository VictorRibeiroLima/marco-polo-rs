use std::sync::Arc;

use marco_polo_rs_core::{
    database::create_pool,
    env,
    internals::{
        cloud::{
            aws::AwsCloudService,
            models::payload::PayloadType,
            traits::{CloudService, QueueClient, QueueMessage},
        },
        subtitler::local::LocalClient,
        transcriber::assembly_ai::AssemblyAiClient,
        translator::google_v2::GoogleTranslateV2Client,
        video_platform::youtube::client::YoutubeClient,
        yt_downloader::yt_dl::YtDl,
    },
};
use sqlx::PgPool;
use tokio::{
    runtime::{Builder, Runtime},
    sync::Mutex,
};
use workers::{heavy::HeavyWorker, light::LightWorker};

use crate::workers::Worker;

mod error;
mod handlers;
mod workers;

pub type CloudServiceInUse = AwsCloudService;
pub type TranscriberClientInUse = AssemblyAiClient;
pub type TranslatorClientInUse = GoogleTranslateV2Client;
pub type SubtitlerClientInUse = LocalClient;
pub type VideoDownloaderInUse = YtDl;
pub type YoutubeClientInUse = YoutubeClient;

pub type Message = <<CloudServiceInUse as CloudService>::QC as QueueClient>::M;

const ERROR_COUNT_THRESHOLD: i64 = 3;
const HEAVY_WORKER_CAPACITY: usize = 1;

struct ServerState {
    cloud_service: CloudServiceInUse,
    runtime: Runtime,
    inactive_light_workers: Arc<Mutex<Vec<LightWorker>>>,
    inactive_heavy_workers: Arc<Mutex<Vec<HeavyWorker>>>,
}

#[tokio::main]
async fn main() {
    let state = start_server_state().await;

    let queue_client = state.cloud_service.queue_client();
    let inactive_light_workers = state.inactive_light_workers;
    let inactive_heavy_workers = state.inactive_heavy_workers;
    let runtime = state.runtime;

    loop {
        let message_result = match queue_client.receive_message().await {
            Ok(messages) => messages,
            Err(e) => {
                eprintln!("Error receiving message:{}", e);
                continue;
            }
        };

        let messages = match message_result {
            Some(messages) => messages,
            _ => {
                continue;
            }
        };

        println!("Enqueuing {} messages", messages.len());
        for message in messages {
            let (message, payload_type) = match get_payload(message, queue_client).await {
                Ok((message, payload_type)) => (message, payload_type),
                Err(_) => continue,
            };
            match payload_type {
                PayloadType::BatukaSrtTranslationUpload(_) => {
                    let pool = inactive_heavy_workers.clone();
                    let message = (message, payload_type);
                    handle_message(&runtime, message, pool).await;
                }
                _ => {
                    let pool = inactive_light_workers.clone();
                    let message = (message, payload_type);
                    handle_message(&runtime, message, pool).await;
                }
            }
        }
    }
}

async fn get_payload<QC: QueueClient>(
    message: QC::M,
    queue_client: &QC,
) -> Result<(QC::M, PayloadType), ()> {
    let payload_result = message.to_payload();
    let payload = match payload_result {
        Ok(payload) => payload,
        Err(_) => {
            println!("Invalid payload");
            let result = queue_client.delete_message(message).await;
            if result.is_err() {
                println!("Failed to delete message");
            }
            return Err(());
        }
    };
    return Ok((message, payload));
}

fn instantiate_worker(
    thread_count: usize,
    pool: Arc<PgPool>,
    cloud_service: CloudServiceInUse,
) -> (Vec<LightWorker>, Vec<HeavyWorker>) {
    let mut inactive_light_workers: Vec<LightWorker> =
        Vec::with_capacity(thread_count - HEAVY_WORKER_CAPACITY);

    let mut inactive_heavy_workers: Vec<HeavyWorker> = Vec::with_capacity(HEAVY_WORKER_CAPACITY);

    for id in 0..thread_count {
        {
            if id < HEAVY_WORKER_CAPACITY {
                let subtitler_client = SubtitlerClientInUse::new();
                let heavy_worker = HeavyWorker {
                    id,
                    pool: pool.clone(),
                    cloud_service: cloud_service.clone(),
                    subtitler_client,
                };
                inactive_heavy_workers.push(heavy_worker);
            } else {
                let translator_client = TranslatorClientInUse::new();
                let transcriber_client = TranscriberClientInUse::new();
                let video_downloader = VideoDownloaderInUse::new();
                let youtube_client = YoutubeClientInUse::new();
                let light_worker = LightWorker {
                    id,
                    pool: pool.clone(),
                    cloud_service: cloud_service.clone(),
                    translator_client,
                    transcriber_client,
                    video_downloader,
                    youtube_client,
                };
                inactive_light_workers.push(light_worker);
            }
        }
    }

    let total_workers = inactive_heavy_workers.len() + inactive_light_workers.len();
    let total_light_workers = inactive_light_workers.len();
    let total_heavy_workers = inactive_heavy_workers.len();

    println!(
        "Created {} workers ({} light, {} heavy)",
        total_workers, total_light_workers, total_heavy_workers
    );

    return (inactive_light_workers, inactive_heavy_workers);
}

async fn handle_message<T: Worker>(
    runtime: &Runtime,
    message: (Message, PayloadType),
    inactive_worker_pool: Arc<Mutex<Vec<T>>>,
) {
    let mut lock = inactive_worker_pool.lock().await;
    let lock_option = lock.pop();
    drop(lock);
    match lock_option {
        Some(worker) => {
            runtime.spawn(async move { worker.handle(message, inactive_worker_pool).await });
        }
        None => {
            return;
        }
    }
}

async fn start_server_state() -> ServerState {
    println!("Starting workers...");
    dotenv::dotenv().ok();
    env::check_envs();
    let thread_count = num_cpus::get_physical();

    println!("Using {} threads", thread_count);

    if thread_count < HEAVY_WORKER_CAPACITY + 1 {
        panic!(
            "Thread count must be at least {}",
            HEAVY_WORKER_CAPACITY + 1
        );
    }

    let pool = create_pool().await;
    let pool = Arc::new(pool);

    let queue_url = std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found");
    let cloud_service = CloudServiceInUse::new(queue_url).unwrap();

    let (inactive_light_workers, inactive_heavy_workers) =
        instantiate_worker(thread_count, pool.clone(), cloud_service.clone());

    let inactive_light_workers = Arc::new(Mutex::new(inactive_light_workers));
    let inactive_heavy_workers = Arc::new(Mutex::new(inactive_heavy_workers));

    let runtime = Builder::new_multi_thread()
        .worker_threads(thread_count)
        .enable_all()
        .build()
        .unwrap();

    return ServerState {
        cloud_service,
        runtime,
        inactive_light_workers,
        inactive_heavy_workers,
    };
}
