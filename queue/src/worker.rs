use std::sync::{Arc, Mutex};

use marco_polo_rs_core::{
    internals::{
        cloud::{
            models::payload::PayloadType,
            traits::{CloudService, QueueClient, QueueMessage},
        },
        subtitler::traits::SubtitlerClient,
        transcriber::traits::TranscriberClient,
        translator::traits::TranslatorClient,
    },
    util::queue::Queue,
};

use super::handlers;

/**
* 1 - Upload the video to Youtube
*/

pub struct Worker<CS, TC, TLC, SC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
    SC: SubtitlerClient<CS::BC>,
{
    pub cloud_service: Arc<CS>,
    pub transcriber_client: Arc<TC>,
    pub translator_client: Arc<TLC>,
    pub subtitler_client: Arc<SC>,
    pub pool: Arc<sqlx::PgPool>,
    pub message_pool: Arc<Mutex<Queue<<<CS as CloudService>::QC as QueueClient>::M>>>,
}

impl<CS, TC, TLC, SC> Worker<CS, TC, TLC, SC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
    SC: SubtitlerClient<CS::BC>,
{
    pub async fn handle_queue(&self) {
        let queue_client = self.cloud_service.queue_client();
        println!("Listening to queue...");
        loop {
            let message_result = queue_client.receive_message().await.unwrap();
            let messages = match message_result {
                Some(messages) => messages,
                _ => {
                    continue;
                }
            };

            for message in messages {
                self.handle_message(message).await;
            }
        }
    }

    async fn handle_message(&self, message: <<CS as CloudService>::QC as QueueClient>::M) {
        let queue_client = self.cloud_service.queue_client();
        let payload_type = match message.to_payload() {
            Ok(payload) => payload,
            Err(_) => {
                return;
            }
        };

        let result = match payload_type {
            PayloadType::BatukaVideoRawUpload(payload) => {
                handlers::raw_upload::handle(&self, payload).await
            }
            PayloadType::BatukaSrtTranscriptionUpload(payload) => {
                let handler = handlers::transcription::Handler::new(&self);
                let sentences_result = handler.handle(payload).await;
                sentences_result
            }
            PayloadType::BatukaSrtTranslationUpload(payload) => {
                let handler = handlers::translation::Handler::new(&self, &message);
                let sentences_result = handler.handle(payload).await;
                sentences_result
            }
            PayloadType::BatukaVideoProcessedUpload(_) => Ok(()),
        };

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("worker result err:{:?}", e);
                return;
            }
        }

        let result = queue_client.delete_message(message).await; //TODO: see what to when delete fails

        match result {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
