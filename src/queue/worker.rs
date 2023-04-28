use crate::{
    database::{
        models::video_storage::VideoFormat,
        queries::{
            self,
            video::{CreateVideoTranscriptionDto, CreateVideoUploadDto},
        },
    },
    internals::{
        cloud::{
            models::payload::{PayloadType, UploadPayload},
            traits::{BucketClient, CloudService, QueueClient, QueueMessage},
        },
        transcriber::traits::TranscriberClient,
    },
};

/**
 * 1 - Call DeepL to translate the transcription
 * 2 - Save the translated transcription in the database
 * 3 - Call FFMPEG (or something else) to generate the video with the translated transcription
 * 4 - Upload the video to Youtube
 */

pub struct Worker<CS, TC>
where
    CS: CloudService,
    TC: TranscriberClient,
{
    pub cloud_service: CS,
    pub transcriber_client: TC,
    pub pool: sqlx::PgPool,
}

impl<CS, TC> Worker<CS, TC>
where
    CS: CloudService,
    TC: TranscriberClient,
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
                    PayloadType::BatukaVideoUpload(payload) => self.handle_upload(payload).await,
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

    async fn handle_upload(
        &self,
        payload: UploadPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bucket_client = self.cloud_service.bucket_client();
        queries::video::create_upload(
            &self.pool,
            CreateVideoUploadDto {
                format: VideoFormat::Mp4,
                storage_id: CS::id(),
                video_id: payload.video_id,
                video_uri: &payload.video_uri,
            },
        )
        .await?;

        let signed_url = bucket_client
            .create_signed_download_url(&payload.video_uri, None)
            .await?;

        let transcribe_id = self.transcriber_client.transcribe(&signed_url).await?;

        queries::video::create_transcription(
            &self.pool,
            CreateVideoTranscriptionDto {
                video_id: payload.video_id,
                transcription_id: transcribe_id,
                transcriber_id: TC::id(),
            },
        )
        .await?;

        Ok(())
    }
}
