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
        translator::{deepl::DeeplClient, google_v2::GoogleTranslateV2Client},
        youtube_client::client::YoutubeClient,
        yt_downloader::yt_dl::YtDl,
    },
    util::queue::Queue,
};
use sqlx::PgPool;
use tokio::{
    runtime::{Builder, Runtime},
    sync::Mutex,
    task::JoinHandle,
};
use workers::{heavy::HeavyWorker, light::LightWorker, Worker};

mod handlers;
mod workers;

pub type CloudServiceInUse = AwsCloudService;
pub type TranscriberClientInUse = AssemblyAiClient;
pub type TranslatorClientInUse = GoogleTranslateV2Client;
pub type SubtitlerClientInUse = LocalClient;
pub type VideoDownloaderInUse = YtDl;
pub type YoutubeClientInUse = YoutubeClient;

pub type Message = <<CloudServiceInUse as CloudService>::QC as QueueClient>::M;

#[tokio::main]
async fn main() {
    println!("Starting workers...");
    dotenv::dotenv().ok();
    env::check_envs();

    let thread_count = num_cpus::get_physical();

    println!("Using {} threads", thread_count);

    if thread_count < 2 {
        panic!("Thread count must be at least 2");
    }

    let pool = create_pool().await;
    let pool = Arc::new(pool);

    let queue_url = std::env::var("AWS_QUEUE_URL").expect("QUEUE_URL not found");
    let cloud_service = CloudServiceInUse::new(queue_url).unwrap();

    let light_message_pool: Queue<(Message, PayloadType)> = Queue::new();
    let light_message_pool = Arc::new(Mutex::new(light_message_pool));

    let heavy_message_pool: Queue<(Message, PayloadType)> = Queue::with_capacity(1);
    let heavy_message_pool = Arc::new(Mutex::new(heavy_message_pool));

    let runtime = Builder::new_multi_thread()
        .worker_threads(thread_count)
        .enable_all()
        .build()
        .unwrap();

    spawn_workers(
        thread_count,
        pool,
        &runtime,
        cloud_service.clone(),
        light_message_pool.clone(),
        heavy_message_pool.clone(),
    );

    let queue_client = cloud_service.queue_client();
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
                    let mut lock = heavy_message_pool.lock().await;
                    lock.enqueue((message, payload_type));
                }
                _ => {
                    let mut lock = light_message_pool.lock().await;
                    lock.enqueue((message, payload_type));
                }
            }
        }
    }
}

fn spawn_workers(
    thread_count: usize,
    pool: Arc<PgPool>,
    runtime: &Runtime,
    cloud_service: CloudServiceInUse,
    light_message_pool: Arc<Mutex<Queue<(Message, PayloadType)>>>,
    heavy_message_pool: Arc<Mutex<Queue<(Message, PayloadType)>>>,
) -> Vec<JoinHandle<()>> {
    let handles: Vec<JoinHandle<()>> = (0..thread_count)
        .map(|id| {
            if id == 0 {
                let subtitler_client = SubtitlerClientInUse::new();
                let heavy_worker = HeavyWorker {
                    id,
                    pool: pool.clone(),
                    cloud_service: cloud_service.clone(),
                    subtitler_client,
                    message_pool: heavy_message_pool.clone(),
                };
                runtime.spawn(async move {
                    heavy_worker.handle_queue().await;
                })
            } else {
                let transcriber_client = TranscriberClientInUse::new();
                let translator_client = TranslatorClientInUse::new();

                let video_downloader = VideoDownloaderInUse::new();
                let youtube_client = YoutubeClientInUse::new();

                let worker = LightWorker {
                    id,
                    pool: pool.clone(),
                    cloud_service: cloud_service.clone(),
                    transcriber_client,
                    translator_client,
                    message_pool: light_message_pool.clone(),
                    video_downloader,
                    youtube_client,
                };

                runtime.spawn(async move {
                    worker.handle_queue().await;
                })
            }
        })
        .collect();

    return handles;
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
