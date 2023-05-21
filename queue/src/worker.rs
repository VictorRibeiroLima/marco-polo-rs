use std::sync::Arc;

use marco_polo_rs_core::{
    internals::cloud::{
        models::payload::PayloadType,
        traits::{CloudService, QueueClient, QueueMessage},
    },
    util::queue::Queue,
};
use tokio::sync::Mutex;

use crate::{
    CloudServiceInUse, Message, SubtitlerClientInUse, TranscriberClientInUse,
    TranslatorClientInUse, VideoDownloaderInUse,
};

use super::handlers;

/**
* 1 - Upload the video to Youtube
*/
pub struct Worker {
    pub id: usize,
    pub cloud_service: CloudServiceInUse,
    pub transcriber_client: TranscriberClientInUse,
    pub translator_client: TranslatorClientInUse,
    pub subtitler_client: SubtitlerClientInUse,
    pub pool: Arc<sqlx::PgPool>,
    pub message_pool: Arc<Mutex<Queue<Message>>>,
    pub video_downloader: VideoDownloaderInUse,
}

impl Worker {
    pub async fn handle_queue(&self) {
        println!("Worker {} started", self.id);
        loop {
            let mut messages = self.message_pool.lock().await;
            let message_result = messages.dequeue();
            drop(messages);

            let message = match message_result {
                Some(message) => message,
                _ => {
                    continue;
                }
            };

            self.handle_message(message).await;
        }
    }

    async fn handle_message(&self, message: Message) {
        let queue_client = self.cloud_service.queue_client();
        let payload_type = match message.to_payload() {
            Ok(payload) => payload,
            Err(_) => {
                let delete_result = queue_client.delete_message(message).await;
                match delete_result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Worker {} delete error: {:?}", self.id, e);
                    }
                }
                return;
            }
        };

        let result = match payload_type {
            PayloadType::BatukaVideoRawUpload(payload) => {
                println!("Worker {} handling raw upload...", self.id);
                handlers::raw_upload::handle(&self, payload).await
            }
            PayloadType::BatukaSrtTranscriptionUpload(payload) => {
                println!("Worker {} handling transcription upload...", self.id);
                let handler = handlers::transcription::Handler::new(&self);
                let sentences_result = handler.handle(payload).await;
                sentences_result
            }
            PayloadType::BatukaSrtTranslationUpload(payload) => {
                println!("Worker {} handling translation upload...", self.id);
                let handler = handlers::translation::Handler::new(&self, &message);
                let sentences_result = handler.handle(payload).await;
                sentences_result
            }
            PayloadType::BatukaVideoProcessedUpload(_) => Ok(()),
            PayloadType::BatukaDownloadVideo(payload) => Ok(()),
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Worker {} error: {:?}", self.id, e);
                return;
            }
        }

        //TODO: see what to when delete fails
        let result = queue_client.delete_message(message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Worker {} delete error: {:?}", self.id, e);
                return;
            }
        }
    }
}
