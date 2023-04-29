use crate::{
    database::{
        models::video_storage::VideoFormat,
        queries::{self, transcription::CreateTranscriptionDto, video::CreateVideoUploadDto},
    },
    internals::{
        cloud::{
            models::payload::UploadPayload,
            traits::{BucketClient, CloudService},
        },
        transcriber::traits::TranscriberClient, translator::traits::TranslatorClient,
    },
    queue::worker::Worker,
};

pub async fn handle<CS, TC, TLC>(
    worker: &Worker<CS, TC, TLC>,
    payload: UploadPayload,
) -> Result<(), Box<dyn std::error::Error>>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
{
    let bucket_client = worker.cloud_service.bucket_client();
    queries::video::create_upload(
        &worker.pool,
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

    let transcribe_id = worker.transcriber_client.transcribe(&signed_url).await?;

    queries::transcription::create(
        &worker.pool,
        CreateTranscriptionDto {
            video_id: payload.video_id,
            transcription_id: transcribe_id,
            transcriber_id: TC::id(),
        },
    )
    .await?;

    Ok(())
}
