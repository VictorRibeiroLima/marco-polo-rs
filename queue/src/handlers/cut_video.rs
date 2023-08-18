use std::path::PathBuf;

use marco_polo_rs_core::{
    database::{
        models::{video::stage::VideoStage, video_storage::StorageVideoStage},
        queries::{self, storage::CreateStorageDto},
    },
    internals::{
        cloud::{
            models::payload::VideoCutPayload,
            traits::{BucketClient, CloudService, QueueClient},
        },
        ServiceProvider,
    },
    util::{ffmpeg, fs},
};

use crate::error::HandlerError;

pub async fn handle<CS: CloudService>(
    payload: VideoCutPayload,
    cloud_service: &CS,
    pool: &sqlx::PgPool,
    message: &<<CS as CloudService>::QC as QueueClient>::M,
) -> Result<(), HandlerError> {
    let video_id = payload.video_id;
    let format = payload.video_format.clone();
    let format_extension = format.to_string();
    let original_file_path = payload.file_path;
    let video_uri = format!("videos/raw/{}.{}", video_id, format_extension);

    let video = match queries::video::find_by_id(pool, &video_id).await {
        Ok(video) => video,
        Err(e) => {
            eprintln!("Failed to find video: {}", e);
            return Err(HandlerError::Final(e.into()));
        }
    };

    let original_id = video.original_video_id;

    queries::video::change_stage(pool, &video_id, VideoStage::Cutting).await?;

    let end_time = match video.end_time {
        Some(end_time) => end_time,
        None => {
            queries::video::change_error_state(pool, &video.id, true).await?; //Need to change this so for the delete_original_file function
            delete_original_file(pool, original_id, &original_file_path).await?;
            eprintln!("Video {} has no end time", video.id);
            return Err(HandlerError::Final("Video has no end time".into()));
        }
    };

    let start_time = video.start_time;

    let raw_path = PathBuf::from(&original_file_path);

    cloud_service
        .queue_client()
        .change_message_visibility(message, 2000) // TODO: Make this configurable
        .await?;

    let cut_output = match ffmpeg::cut_video(&raw_path, &start_time, &end_time) {
        Ok(output) => output,
        Err(e) => {
            queries::video::change_error_state(pool, &video.id, true).await?; //Need to change this so for the delete_original_file function
            delete_original_file(pool, original_id, &original_file_path).await?;
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

    let mut trx = pool.begin().await?;

    let storage_dto = CreateStorageDto {
        video_id: &video_id,
        format,
        video_uri: &video_uri,
        storage_id: cloud_service.bucket_client().id(),
        stage: StorageVideoStage::Raw,
        size: cut_size as i64, //if some day a negative value appears on the database, this is the reason
    };

    queries::storage::create(&mut *trx, storage_dto).await?;

    queries::video::change_stage(&mut *trx, &video_id, VideoStage::RawUploading).await?;

    cloud_service
        .bucket_client()
        .upload_file_from_path(&video_uri, &cut_output)
        .await?;

    trx.commit().await?;

    delete_original_file(pool, original_id, &original_file_path).await?;
    std::fs::remove_file(cut_output)?;

    return Ok(());
}

async fn delete_original_file(
    pool: &sqlx::PgPool,
    original_id: i32,
    original_file_path: &str,
) -> Result<(), HandlerError> {
    let original_video_count = queries::original_video::count_videos(pool, original_id).await?;
    let finished_video_count =
        queries::original_video::count_finished_cuts(pool, original_id).await?;

    if original_video_count == finished_video_count {
        match std::fs::remove_file(original_file_path) {
            Ok(_) => (),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => {
                    eprintln!("Failed to delete original file: {}", e);
                    return Err(HandlerError::Final(e.into()));
                }
            },
        }
    }

    Ok(())
}
