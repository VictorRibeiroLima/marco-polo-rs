use marco_polo_rs_core::{
    database::{
        models::{user::VideoStage, video_storage::StorageVideoStage},
        queries::{self, storage::CreateStorageDto},
    },
    internals::{
        cloud::{
            models::payload::VideoDownloadPayload,
            traits::{BucketClient, CloudService, QueueClient},
        },
        yt_downloader::traits::YoutubeDownloader,
        ServiceProvider,
    },
    SyncError,
};

use crate::{worker::Worker, Message};

pub async fn handle(
    payload: VideoDownloadPayload,
    worker: &Worker,
    message: &Message,
) -> Result<(), SyncError> {
    let video_id = payload.video_id;
    let format = payload.video_format.clone();
    let format_extension = format.to_string();
    let video_uri = format!("videos/raw/{}.{}", video_id, format_extension);

    worker
        .cloud_service
        .queue_client()
        .change_message_visibility(message, 4000) // TODO: Make this configurable
        .await?;

    let output_file = worker.video_downloader.download(payload.into()).await?;

    worker
        .cloud_service
        .bucket_client
        .upload_file_from_path(&video_uri, &output_file)
        .await?;

    std::fs::remove_file(output_file)?;

    let storage_dto = CreateStorageDto {
        video_id: &video_id,
        format,
        video_uri: &video_uri,
        storage_id: worker.cloud_service.bucket_client.id(),
        stage: StorageVideoStage::Raw,
    };

    queries::storage::create(&worker.pool, storage_dto).await?;

    queries::video::change_stage(&worker.pool, &video_id, VideoStage::Transcribing).await?;

    Ok(())
}
