use marco_polo_rs_core::{
    database::{
        models::video_storage::StorageVideoStage,
        queries::{self, storage::CreateStorageDto},
    },
    internals::{
        cloud::{models::payload::VideoDownloadPayload, traits::BucketClient},
        yt_downloader::traits::YoutubeDownloader,
        ServiceProvider,
    },
    SyncError,
};

use crate::worker::Worker;

pub async fn handle(payload: VideoDownloadPayload, worker: &Worker) -> Result<(), SyncError> {
    let video_id = payload.video_id;
    let format = payload.video_format.clone();
    let format_extension = format.to_string();
    let video_uri = format!("videos/raw/{}.{}", video_id, format_extension);

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

    Ok(())
}
