use marco_polo_rs_core::{
    database::{
        models::{video::stage::VideoStage, video_storage::StorageVideoStage},
        queries::{self},
    },
    internals::{cloud::models::payload::VideoPayload, youtube_client::traits::YoutubeClient},
};
use sqlx::PgPool;

use crate::{error::HandlerError, YoutubeClientInUse};

pub async fn handle(
    pool: &PgPool,
    youtube_client: &YoutubeClientInUse,
    payload: VideoPayload,
) -> Result<(), HandlerError> {
    let video_with_storage_and_channel = queries::video::find_by_id_with_storage_and_channel(
        &pool,
        &payload.video_id,
        StorageVideoStage::Processed,
    )
    .await?;

    queries::video::change_stage(pool, &payload.video_id, VideoStage::Uploading).await?;

    let youtube_video = youtube_client
        .upload_video(&video_with_storage_and_channel)
        .await?;

    let video_url = format!(
        "https://www.youtube.com/watch?v={}",
        youtube_video.id.unwrap()
    );

    queries::video::set_url(pool, &video_with_storage_and_channel.video.id, video_url).await?;
    Ok(())
}
