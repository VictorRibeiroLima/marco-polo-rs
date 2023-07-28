use std::sync::Arc;

use marco_polo_rs_core::{
    internals::cloud::{
        models::payload::PayloadType,
        traits::{CloudService, QueueClient},
    },
    util::queue::Queue,
    SyncError,
};
use tokio::sync::Mutex;

use crate::{
    error::HandlerError,
    handlers::{download_video, processed_upload, raw_upload, transcription},
    CloudServiceInUse, Message, TranscriberClientInUse, TranslatorClientInUse,
    VideoDownloaderInUse, YoutubeClientInUse,
};

use super::Worker;

pub struct LightWorker {
    pub id: usize,
    pub cloud_service: CloudServiceInUse,
    pub transcriber_client: TranscriberClientInUse,
    pub translator_client: TranslatorClientInUse,
    pub pool: Arc<sqlx::PgPool>,
    pub message_pool: Arc<Mutex<Queue<(Message, PayloadType)>>>,
    pub video_downloader: VideoDownloaderInUse,
    pub youtube_client: YoutubeClientInUse,
}

impl LightWorker {
    async fn handle_message(&self, message: Message, payload_type: PayloadType) {
        let queue_client = self.cloud_service.queue_client();

        let result: Result<(), HandlerError> = self.handle_payload(payload_type, &message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Light Worker {} error: {:?}", self.id, e);
                match e {
                    HandlerError::Retrievable(_) => {
                        return;
                    }
                    HandlerError::Final(_) => {
                        self.delete_message(queue_client, message).await;
                        return;
                    }
                }
            }
        }

        self.delete_message(queue_client, message).await;
    }

    async fn handle_payload(
        &self,
        payload_type: PayloadType,
        message: &Message,
    ) -> Result<(), HandlerError> {
        match payload_type {
            PayloadType::BatukaVideoRawUpload(payload) => {
                println!("Light Worker {} handling raw upload...", self.id);
                let result: Result<(), HandlerError> = raw_upload::handle(
                    &self.cloud_service,
                    &self.transcriber_client,
                    &self.pool,
                    &message,
                    payload,
                )
                .await;

                return result;
            }

            PayloadType::BatukaSrtTranscriptionUpload(payload) => {
                println!("Light Worker {} handling transcription upload...", self.id);
                let handler = transcription::Handler::new(
                    &self.transcriber_client,
                    &self.cloud_service,
                    &self.translator_client,
                    self.pool.clone(),
                );

                return handler.handle(payload).await;
            }

            PayloadType::BatukaVideoProcessedUpload(payload) => {
                println!("Light Worker {} handling processed upload...", self.id);
                return processed_upload::handle(&self.pool, &self.youtube_client, payload).await;
            }

            //Maybe heavy worker should handle this
            PayloadType::BatukaDownloadVideo(payload) => {
                println!("Light Worker {} handling video download...", self.id);
                let download_result: Result<(), HandlerError> = download_video::handle(
                    payload,
                    &self.cloud_service,
                    &self.video_downloader,
                    &self.pool,
                    message,
                )
                .await;
                return download_result;
            }

            PayloadType::BatukaSrtTranslationUpload(_) => {
                panic!("Light worker should not handle translation uploads")
            }
        };
    }

    async fn delete_message<QC: QueueClient>(
        &self,
        queue_client: &QC,
        message: <QC as QueueClient>::M,
    ) {
        let result: Result<(), SyncError> = queue_client.delete_message(message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Light Worker {} delete error: {:?}", self.id, e);
                return;
            }
        }
    }
}

#[async_trait::async_trait]
impl Worker for LightWorker {
    async fn handle_queue(&self) {
        println!("Light Worker {} started", self.id);
        loop {
            let mut messages = self.message_pool.lock().await;
            let dequeue_result = messages.dequeue();
            drop(messages);

            let (message, payload_type) = match dequeue_result {
                Some((message, payload_type)) => (message, payload_type),
                _ => {
                    continue;
                }
            };

            self.handle_message(message, payload_type).await;
        }
    }
}
