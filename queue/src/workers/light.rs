use std::sync::Arc;

use marco_polo_rs_core::{
    database::queries::{self, video::CreateErrorDto},
    internals::cloud::{
        models::payload::PayloadType,
        traits::{CloudService, QueueClient},
    },
    SyncError,
};
use tokio::sync::Mutex;

use crate::{
    error::HandlerError,
    handlers::{download_video, processed_upload, raw_upload, transcription},
    CloudServiceInUse, Message, TranscriberClientInUse, TranslatorClientInUse,
    VideoDownloaderInUse, YoutubeClientInUse, ERROR_COUNT_THRESHOLD,
};

use super::Worker;

pub struct LightWorker {
    pub id: usize,
    pub cloud_service: CloudServiceInUse,
    pub transcriber_client: TranscriberClientInUse,
    pub translator_client: TranslatorClientInUse,
    pub pool: Arc<sqlx::PgPool>,
    pub video_downloader: VideoDownloaderInUse,
    pub youtube_client: YoutubeClientInUse,
}

impl LightWorker {
    async fn handle_message(&self, message: Message, payload_type: PayloadType) {
        let queue_client = self.cloud_service.queue_client();
        let video_id = payload_type.video_id();

        let result: Result<(), HandlerError> = self.handle_payload(payload_type, &message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Light Worker {} error: {:?}", self.id, e);
                let dto = CreateErrorDto {
                    video_id: &video_id,
                    error: &e.to_string(),
                };
                let error_count = match queries::video::create_error(&self.pool, dto).await {
                    Ok(count) => count,
                    Err(e) => {
                        println!("Light Worker {} error: {:?}", self.id, e);
                        self.delete_message(queue_client, message).await;
                        return;
                    }
                };
                match e {
                    HandlerError::Retrievable(_) => {
                        if error_count >= ERROR_COUNT_THRESHOLD {
                            self.delete_message(queue_client, message).await;
                        }
                        return;
                    }
                    HandlerError::Final(_) => {
                        self.delete_message(queue_client, message).await;
                        return;
                    }
                }
            } //TODO: better control flow
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
    async fn handle(
        self,
        message: (Message, PayloadType),
        inactive_worker_pool: Arc<Mutex<Vec<LightWorker>>>,
    ) {
        println!("Light Worker {} is now active", self.id);
        let (message, payload_type) = message;
        self.handle_message(message, payload_type).await;

        println!("Light Worker {} is now inactive", self.id);
        let mut pool = inactive_worker_pool.lock().await;
        println!(
            "Light Worker {} is now putting itself back in the pool",
            self.id
        );
        pool.push(self);
    }
}
