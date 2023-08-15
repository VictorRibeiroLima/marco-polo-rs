use marco_polo_rs_core::{
    database::{
        models::video::stage::VideoStage,
        queries::{self, transcription::CreateTranscriptionDto},
    },
    internals::{
        cloud::{
            models::payload::VideoPayload,
            traits::{BucketClient, CloudService, QueueClient},
        },
        transcriber::traits::TranscriberClient,
    },
};

use crate::error::HandlerError;

pub async fn handle<CS: CloudService>(
    cloud_service: &CS,
    transcriber_client: &impl TranscriberClient,
    pool: &sqlx::PgPool,
    message: &<<CS as CloudService>::QC as QueueClient>::M,
    payload: VideoPayload,
) -> Result<(), HandlerError> {
    let bucket_client = cloud_service.bucket_client();

    let queue_client = cloud_service.queue_client();

    queue_client.change_message_visibility(message, 100).await?;

    let signed_url = bucket_client
        .create_signed_download_url(&payload.video_uri, None)
        .await?;

    queries::video::change_stage(pool, &payload.video_id, VideoStage::Transcribing).await?;

    let transcribe_id = transcriber_client.transcribe(&signed_url).await?;

    queries::transcription::create(
        pool,
        CreateTranscriptionDto {
            video_id: payload.video_id,
            transcription_id: transcribe_id,
            transcriber_id: transcriber_client.id(),
        },
    )
    .await?;

    Ok(())
}
