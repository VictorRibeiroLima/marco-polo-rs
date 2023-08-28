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
    let video = queries::video::with_channel::with_channels(&pool, payload.video_id).await?;

    //TODO: this can be done in a single query, but I'm too lazy to do it right now
    let storage = queries::storage::find_by_video_id_and_stage(
        &pool,
        &video.video.id,
        StorageVideoStage::Processed,
    )
    .await?;

    queries::video::change_stage(pool, &payload.video_id, VideoStage::Uploading).await?;

    //TODO: Make Promise.all
    for channel in &video.channels {
        let upload_params = UploadParams {
            video: &video.video,
            storage: &storage,
            channel,
        };

        match channel.platform {
            Platform::Youtube => {
                youtube_upload(upload_params, youtube_client, pool).await?;
            }
            _ => {
                return Err(HandlerError::Final("Unsupported platform".into()));
            }
        };
    }

    Ok(())
}

async fn youtube_upload(
    video: UploadParams<'_>,
    youtube_client: &YoutubeClientInUse,
    pool: &PgPool,
) -> Result<(), HandlerError> {
    let video_id = video.video.id;
    let channel_id = video.channel.id;
    let youtube_video = youtube_client.upload_video(video).await?;

    let video_url = format!(
        "https://www.youtube.com/watch?v={}",
        youtube_video.id.unwrap()
    );

    let mut trx = pool.begin().await?;

    queries::video_channel::set_url(&mut *trx, video_id, channel_id, video_url).await?;
    queries::video::change_stage(&mut *trx, &video_id, VideoStage::Done).await?;

    trx.commit().await?;

    Ok(())
}
