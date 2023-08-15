use marco_polo_rs_core::{
    database::{
        models::video_storage::StorageVideoStage,
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
    util::{ffmpeg, fs},
};

use crate::error::HandlerError;

pub async fn handle<CS: CloudService>(
    payload: VideoDownloadPayload,
    cloud_service: &CS,
    video_downloader: &impl YoutubeDownloader,
    pool: &sqlx::PgPool,
    message: &<<CS as CloudService>::QC as QueueClient>::M,
) -> Result<(), HandlerError> {
    let video_id = payload.video_id;
    let format = payload.video_format.clone();
    let format_extension = format.to_string();
    let video_uri = format!("videos/raw/{}.{}", video_id, format_extension);

    let estimated_time = video_downloader.estimate_time(&payload.video_url).await?;

    cloud_service
        .queue_client()
        .change_message_visibility(message, estimated_time) // TODO: Make this configurable
        .await?;

    let output_file = video_downloader.download(&payload.video_url).await?;

    let raw_path = std::path::PathBuf::from(&output_file);

    let original_video_duration = match ffmpeg::get_video_duration(&raw_path) {
        Ok(duration) => duration,
        Err(e) => {
            std::fs::remove_file(output_file)?;
            return Err(HandlerError::Final(e.into()));
        }
    };

    let end_time = match &payload.end_time {
        Some(end_time) => end_time,
        None => &original_video_duration,
    };

    let start_time = match &payload.start_time {
        Some(start_time) => start_time,
        None => "00:00:00",
    };

    let cut_output = match ffmpeg::cut_video(&raw_path, &start_time, &end_time) {
        Ok(output) => output,
        Err(e) => {
            std::fs::remove_file(output_file)?;
            return Err(HandlerError::Final(e.into()));
        }
    };

    let cut_path = std::path::PathBuf::from(&cut_output);

    let cut_size = match fs::check_file_size(&cut_path) {
        Ok(size) => size,
        Err(e) => {
            eprintln!("Failed to check file size: {}", e);
            0
        }
    };

    cloud_service
        .bucket_client()
        .upload_file_from_path(&video_uri, &cut_output)
        .await?;

    std::fs::remove_file(output_file)?;
    std::fs::remove_file(cut_output)?;

    let storage_dto = CreateStorageDto {
        video_id: &video_id,
        format,
        video_uri: &video_uri,
        storage_id: cloud_service.bucket_client().id(),
        stage: StorageVideoStage::Raw,
        size: cut_size as i64, //if some day a negative value appears on the database, this is the reason
    };

    queries::storage::create(pool, storage_dto).await?;

    //DESCOMENTAR
    //queries::video::update_metadata(pool, &video_id, &original_video_duration, end_time).await?;

    Ok(())
}
