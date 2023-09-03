use marco_polo_rs_core::{
    database::{
        models::{
            channel::platform::Platform, video::stage::VideoStage, video_storage::StorageVideoStage,
        },
        queries::{self},
    },
    internals::{
        cloud::models::payload::VideoPayload,
        video_platform::{UploadParams, VideoPlatformClient},
    },
};
use sqlx::PgPool;

use crate::{error::HandlerError, YoutubeClientInUse};

pub async fn handle(
    pool: &PgPool,
    youtube_client: &YoutubeClientInUse,
    payload: VideoPayload,
) -> Result<(), HandlerError> {
    let video = queries::video::find_by_id_with_storage_and_channel(
        pool,
        &payload.video_id,
        StorageVideoStage::Processed,
    )
    .await?;

    queries::video::change_stage(pool, &payload.video_id, VideoStage::Uploading).await?;

    let storage = video.storage;
    let channel = video.channel;
    let video = video.video;

    let upload_params = UploadParams {
        video: &video,
        storage: &storage,
        channel: &channel,
    };

    match channel.platform {
        Platform::Youtube => {
            youtube_upload(upload_params, youtube_client, pool).await?;
        }
        _ => {
            return Err(HandlerError::Final("Unsupported platform".into()));
        }
    };

    Ok(())
}

async fn youtube_upload(
    video: UploadParams<'_>,
    youtube_client: &YoutubeClientInUse,
    pool: &PgPool,
) -> Result<(), HandlerError> {
    let video_id = video.video.id;
    let youtube_video = youtube_client.upload_video(video).await?;

    let video_url = format!(
        "https://www.youtube.com/watch?v={}",
        youtube_video.id.unwrap()
    );

    queries::video::set_url(pool, video_id, &video_url).await?;

    Ok(())
}
