use std::sync::Arc;

use marco_polo_rs_core::{
    database::queries::{self, video::CreateErrorDto},
    internals::cloud::{
        models::payload::PayloadType,
        traits::{CloudService, QueueClient},
    },
    util::queue::Queue,
    SyncError,
};
use tokio::sync::Mutex;

use crate::{
    error::HandlerError, handlers, CloudServiceInUse, Message, SubtitlerClientInUse,
    ERROR_COUNT_THRESHOLD,
};

use super::Worker;

pub struct HeavyWorker {
    pub id: usize,
    pub message_pool: Arc<Mutex<Queue<(Message, PayloadType)>>>,
    pub cloud_service: CloudServiceInUse,
    pub subtitler_client: SubtitlerClientInUse,
    pub pool: Arc<sqlx::PgPool>,
}

impl HeavyWorker {
    async fn delete_message<QC: QueueClient>(
        &self,
        queue_client: &QC,
        message: <QC as QueueClient>::M,
    ) {
        let result: Result<(), SyncError> = queue_client.delete_message(message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Heavy Worker {} delete error: {:?}", self.id, e);
                return;
            }
        }
    }

    async fn handle_message(&self, message: Message, payload_type: PayloadType) {
        let queue_client = self.cloud_service.queue_client();

        let video_id = payload_type.video_id();
        let result = self.handle_payload(payload_type, &message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Heavy Worker {} error: {:?}", self.id, e);
                let dto = CreateErrorDto {
                    video_id: &video_id,
                    error: &e.to_string(),
                };
                let error_count = queries::video::create_error(&self.pool, dto).await.unwrap(); //TODO: unwrap
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
                };
            }
        }

        //TODO: see what to when delete fails
        self.delete_message(queue_client, message).await;
    }

    async fn handle_payload(
        &self,
        payload_type: PayloadType,
        message: &Message,
    ) -> Result<(), HandlerError> {
        match payload_type {
            PayloadType::BatukaSrtTranslationUpload(payload) => {
                println!("Heavy Worker {} handling translation upload...", self.id);

                let handler = handlers::translation::Handler::new(
                    &self.cloud_service,
                    &self.subtitler_client,
                    self.pool.clone(),
                    &message,
                );

                let sentences_result: Result<(), HandlerError> = handler.handle(payload).await;
                return sentences_result;
            }

            _ => {
                panic!("Heavy worker should only handle translation uploads")
            }
        };
    }
}

#[async_trait::async_trait]
impl Worker for HeavyWorker {
    async fn handle_queue(&self) {
        println!("Heavy Worker {} started", self.id);
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
