use crate::internals::{
    cloud::{
        aws::payload::PayloadType,
        traits::{BucketClient, QueueClient, QueueMessage},
    },
    transcriber::traits::TranscriberClient,
};

pub async fn handle_uploaded_file<QC, QM, BC>(queue_client: QC, bucket_client: BC)
where
    QC: QueueClient<QM>,
    QM: QueueMessage + std::fmt::Debug,
    BC: BucketClient,
{
    loop {
        let message_result = queue_client.receive_message().await.unwrap();
        let messages = match message_result {
            Some(messages) => messages,
            _ => {
                continue;
            }
        };

        for message in messages {
            let body = message.get_message();
            if body.is_empty() {
                continue;
            }

            let payload_type = match PayloadType::from_str(body.as_str()) {
                Ok(payload) => payload,
                Err(_) => {
                    continue;
                }
            };

            let payload = match payload_type {
                PayloadType::BatukaVideoUpload(payload) => payload,
            };

            let signed_url = bucket_client
                .create_signed_download_url(&payload.s3video_uri, None)
                .unwrap();

            let transcriber_client =
                crate::internals::transcriber::assembly_ai::AssemblyAiClient::new();
            let transcribe_id = match transcriber_client.transcribe(&signed_url).await {
                Ok(id) => id,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };

            println!("{:?}", transcribe_id);

            /**
             * 1 - Generate signed URL for the file
             * 2 - Call assemblyAI to transcribe the file
             * 3 - Save the original transcription in the database
             * 4 - Call DeepL to translate the transcription
             * 5 - Save the translated transcription in the database
             * 6 - Call FFMPEG (or something else) to generate the video with the translated transcription
             * 7 - Upload the video to Youtube
             */
            let delete_result = queue_client.delete_message(message).await; //TODO: see what to when delete fails

            match delete_result {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}
