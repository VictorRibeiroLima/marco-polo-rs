use marco_polo_rs_core::{
    database::queries::{self, transcription::CreateTranscriptionDto},
    internals::{
        cloud::{
            models::payload::VideoPayload,
            traits::{BucketClient, CloudService},
        },
        transcriber::traits::TranscriberClient,
        ServiceProvider,
    },
    SyncError,
};

use crate::worker::Worker;

pub async fn handle(worker: &Worker, payload: VideoPayload) -> Result<(), SyncError> {
    let bucket_client = worker.cloud_service.bucket_client();

    let signed_url = bucket_client
        .create_signed_download_url(&payload.video_uri, None)
        .await?;

    let transcribe_id = worker.transcriber_client.transcribe(&signed_url).await?;

    queries::transcription::create(
        &worker.pool,
        CreateTranscriptionDto {
            video_id: payload.video_id,
            transcription_id: transcribe_id,
            transcriber_id: worker.transcriber_client.id(),
        },
    )
    .await?;

    Ok(())
}
