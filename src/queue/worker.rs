use std::sync::Arc;

use crate::internals::{
    cloud::{
        models::payload::PayloadType,
        traits::{CloudService, QueueClient, QueueMessage},
    },
    transcriber::traits::TranscriberClient,
    translator::traits::TranslatorClient,
};

use crate::util::time_it;

use super::handlers;

/**
 * 1 - Call DeepL to translate the transcription
 * 2 - Save the translated transcription in the database
 * 3 - Call FFMPEG (or something else) to generate the video with the translated transcription
 * 4 - Upload the video to Youtube
 */

pub struct Worker<CS, TC, TLC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
{
    pub cloud_service: CS,
    pub transcriber_client: TC,
    pub translator_client: TLC,
    pub pool: Arc<sqlx::PgPool>,
}

impl<CS, TC, TLC> Worker<CS, TC, TLC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
{
    pub async fn handle_queue(&self) {
        let queue_client = self.cloud_service.queue_client();

        loop {
            let message_result = queue_client.receive_message().await.unwrap();
            let messages = match message_result {
                Some(messages) => messages,
                _ => {
                    continue;
                }
            };

            for message in messages {
                let payload_type = match message.to_payload() {
                    Ok(payload) => payload,
                    Err(_) => {
                        continue;
                    }
                };

                let result = match payload_type {
                    PayloadType::BatukaVideoUpload(payload) => {
                        handlers::upload::handle(&self, payload).await
                    }
                    PayloadType::BatukaSrtTranscriptionUpload(payload) => {
                        let handler = handlers::transcription::Handler::new(&self);

                        let sentences_result = handler.get_sentences(payload).await;

                        let sentences = match sentences_result {
                            Ok(sentences) => sentences,
                            Err(e) => {
                                println!("{:?}", e);
                                continue;
                            }
                        };

                        let wait_time_in_secs = sentences.len() / 10;

                        let result = queue_client
                            .change_message_visibility(&message, wait_time_in_secs)
                            .await;

                        match result {
                            Ok(_) => {}
                            Err(e) => {
                                println!("{:?}", e);
                                continue;
                            }
                        }
                        let result;
                        time_it!(
                            {
                                result = handler.translate(sentences).await;
                            },
                            as_millis
                        );

                        result
                    }
                };

                match result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{:?}", e);
                        continue;
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
    }
}
