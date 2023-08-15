use marco_polo_rs_core::{
    database::{
        models::{original_video::OriginalVideo, video::Video},
        queries::{self, filter::Filter, pagination},
    },
    internals::{
        cloud::{
            models::payload::VideoDownloadPayload,
            traits::{CloudService, QueueClient},
        },
        yt_downloader::traits::YoutubeDownloader,
    },
    util::ffmpeg,
};

use crate::error::HandlerError;

pub async fn handle<CS: CloudService>(
    payload: VideoDownloadPayload,
    cloud_service: &CS,
    video_downloader: &impl YoutubeDownloader,
    pool: &sqlx::PgPool,
    message: &<<CS as CloudService>::QC as QueueClient>::M,
) -> Result<(), HandlerError> {
    let mut video_filter: Filter<Video> = Default::default();
    video_filter.options.original_video_id = Some(payload.original_video_id);

    let original_video_filter: Filter<OriginalVideo> = Default::default();

    let mut pagination = pagination::Pagination::default();
    pagination.limit = Some(24); //TODO: Make this a const on core

    let videos = queries::video::find_all_with_original(
        pool,
        pagination,
        video_filter,
        original_video_filter,
    )
    .await?;

    let original_video = &videos
        .first()
        .ok_or_else(|| HandlerError::Final("No videos found".into()))?
        .original;

    let original_video_id = original_video.id;

    let estimated_time = video_downloader.estimate_time(&original_video.url).await?;

    cloud_service
        .queue_client()
        .change_message_visibility(message, estimated_time) // TODO: Make this configurable
        .await?;

    let output_file = video_downloader.download(&original_video.url).await?;

    let raw_path = std::path::PathBuf::from(&output_file);

    let original_video_duration = match ffmpeg::get_video_duration(&raw_path) {
        Ok(duration) => duration,
        Err(e) => {
            std::fs::remove_file(output_file)?;
            return Err(HandlerError::Final(e.into()));
        }
    };

    let mut without_end_time_ids = vec![];

    for video in videos {
        let video = video.video;
        match video.end_time {
            Some(_) => continue,
            None => {
                without_end_time_ids.push(video.id);
            }
        }
    }

    queries::video::bulk_update_end_time(pool, without_end_time_ids, &original_video_duration)
        .await?;

    queries::original_video::update_duration(pool, original_video_id, &original_video_duration)
        .await?;

    Ok(())
}
