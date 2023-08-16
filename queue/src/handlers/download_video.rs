use futures::future::try_join_all;
use marco_polo_rs_core::{
    database::{
        models::{original_video::OriginalVideo, video::Video, video_storage::VideoFormat},
        queries::{self, filter::Filter, pagination},
    },
    internals::{
        cloud::{
            models::payload::{PayloadType, VideoCutPayload, VideoDownloadPayload},
            traits::{CloudService, QueueClient},
        },
        yt_downloader::traits::YoutubeDownloader,
    },
    util::ffmpeg,
    MAX_NUMBER_OF_CUTS,
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
    pagination.limit = Some(MAX_NUMBER_OF_CUTS.try_into().unwrap()); //unwrap is safe because MAX_NUMBER_OF_CUTS is a small number

    let videos = queries::video::with_original::find_all_with_original(
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
    let mut messages = vec![];

    for video in videos {
        let mut video = video.video;
        match video.end_time {
            Some(_) => continue,
            None => {
                video.end_time = Some(original_video_duration.to_string());
                without_end_time_ids.push(video.id);
            }
        }

        let payload: VideoCutPayload = VideoCutPayload {
            video_id: video.id,
            video_format: VideoFormat::Mkv,
            file_path: output_file.clone(),
        };

        let payload = PayloadType::BatukaCutVideo(payload);

        let handler = cloud_service.queue_client().send_message(payload);
        messages.push(handler);
    }

    queries::video::bulk_update_end_time(pool, without_end_time_ids, &original_video_duration)
        .await?;

    queries::original_video::update_duration(pool, original_video_id, &original_video_duration)
        .await?;

    try_join_all(messages).await?;

    Ok(())
}
