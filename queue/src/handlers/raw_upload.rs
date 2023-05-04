use marco_polo_rs_core::{
    database::{
        models::video_storage::{VideoFormat, VideoStage},
        queries::{self, transcription::CreateTranscriptionDto, video::CreateVideoWithStorageDto},
    },
    internals::{
        cloud::{
            models::payload::VideoPayload,
            traits::{BucketClient, CloudService},
        },
        subtitler::traits::SubtitlerClient,
        transcriber::traits::TranscriberClient,
        translator::traits::TranslatorClient,
    },
};

use crate::worker::Worker;

pub async fn handle<CS, TC, TLC, SC>(
    worker: &Worker<CS, TC, TLC, SC>,
    payload: VideoPayload,
) -> Result<(), Box<dyn std::error::Error>>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
    SC: SubtitlerClient<CS::BC>,
{
    let bucket_client = worker.cloud_service.bucket_client();
    queries::video::create_with_storage(
        &worker.pool,
        CreateVideoWithStorageDto {
            format: VideoFormat::Mkv,
            storage_id: CS::id(),
            video_id: payload.video_id,
            video_uri: &payload.video_uri,
            stage: VideoStage::Raw,
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
