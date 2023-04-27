use crate::internals::{
    cloud::{
        models::{
            payload::{PayloadType, UploadPayload},
            service::CloudService,
        },
        traits::{BucketClient, QueueClient, QueueMessage},
    },
    transcriber::{assembly_ai::AssemblyAiClient, traits::TranscriberClient},
};

/**
 * 1 - Generate signed URL for the file
 * 2 - Call assemblyAI to transcribe the file
 * 3 - Save the original transcription in the database
 * 4 - Call DeepL to translate the transcription
 * 5 - Save the translated transcription in the database
 * 6 - Call FFMPEG (or something else) to generate the video with the translated transcription
 * 7 - Upload the video to Youtube
 */

pub struct Worker<CS, TC>
where
    CS: CloudService,
    TC: TranscriberClient,
{
    pub cloud_service: CS,
    pub transcriber_client: TC,
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
        let signed_url = bucket_client.create_signed_download_url(&payload.video_uri, None)?;

        let transcribe_id = self.transcriber_client.transcribe(&signed_url).await?;

        println!("{:?}", transcribe_id);
        Ok(())
    }
}
