use futures::future::try_join_all;
use marco_polo_rs_core::{
    database::{models::video_storage::VideoFormat, queries},
    internals::{
        cloud::{
            models::payload::{PayloadType, VideoCutPayload, VideoDownloadPayload},
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
    let id = payload.original_video_id;

    let original_with_video =
        queries::original_video::with_video::find_with_videos(pool, id).await?;

    let original_video = original_with_video.original_video;
    let videos = original_with_video.videos;

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

    for mut video in videos {
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

    queries::original_video::update_duration(pool, original_video.id, &original_video_duration)
        .await?;

    try_join_all(messages).await?;

    Ok(())
}
