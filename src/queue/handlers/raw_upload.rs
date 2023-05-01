use crate::{
    database::{
        models::video_storage::{VideoFormat, VideoStage},
        queries::{self, transcription::CreateTranscriptionDto, video::CreateVideoDto},
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
    queue::worker::Worker,
};

pub async fn handle<CS, TC, TLC, SC>(
    worker: &Worker<CS, TC, TLC, SC>,
    payload: VideoPayload,
) -> Result<(), Box<dyn std::error::Error>>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
    SC: SubtitlerClient,
{
    let bucket_client = worker.cloud_service.bucket_client();
    queries::video::create(
        &worker.pool,
        CreateVideoDto {
            format: VideoFormat::Mp4,
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
