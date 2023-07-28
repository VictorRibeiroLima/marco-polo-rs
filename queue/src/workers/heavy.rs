use std::sync::Arc;

use marco_polo_rs_core::{
    internals::cloud::{
        models::payload::PayloadType,
        traits::{CloudService, QueueClient},
    },
    util::queue::Queue,
};
use tokio::sync::Mutex;

use crate::{error::HandlerError, handlers, CloudServiceInUse, Message, SubtitlerClientInUse};

use super::Worker;

pub struct HeavyWorker {
    pub id: usize,
    pub message_pool: Arc<Mutex<Queue<(Message, PayloadType)>>>,
    pub cloud_service: CloudServiceInUse,
    pub subtitler_client: SubtitlerClientInUse,
    pub pool: Arc<sqlx::PgPool>,
}

impl HeavyWorker {
    async fn handle_message(&self, message: Message, payload_type: PayloadType) {
        let queue_client = self.cloud_service.queue_client();

        let result = self.handle_payload(payload_type, &message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Heavy Worker {} error: {:?}", self.id, e);
                return;
            }
        }

        //TODO: see what to when delete fails
        let result = queue_client.delete_message(message).await;

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Heavy Worker {} delete error: {:?}", self.id, e);
                return;
            }
        }
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
