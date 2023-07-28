use marco_polo_rs_core::{
    database::{
        models::{video::VideoStage, video_storage::StorageVideoStage},
        queries::{self, video::CreateErrorDto},
    },
    internals::{cloud::models::payload::VideoPayload, youtube_client::traits::YoutubeClient},
};
use sqlx::PgPool;

use crate::{error::HandlerError, YoutubeClientInUse, ERROR_COUNT_THRESHOLD};

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

    queries::video::change_stage(&pool, &payload.video_id, VideoStage::Uploading).await?;

    let youtube_video_result = youtube_client
        .upload_video(&video_with_storage_and_channel)
        .await;

    let youtube_video = match youtube_video_result {
        Ok(video) => video,
        Err(error) => {
            let dto = CreateErrorDto {
                video_id: &payload.video_id,
                error: &error.to_string(),
                stage: VideoStage::Uploading,
            };
            let error_count = queries::video::create_error(&pool, dto).await?;
            if error_count >= ERROR_COUNT_THRESHOLD {
                return Err(HandlerError::Final(error));
            }
            return Err(HandlerError::Retry(error));
        }
    };

    let video_url = format!(
        "https://www.youtube.com/watch?v={}",
        youtube_video.id.unwrap()
    );

    queries::video::set_url(pool, &video_with_storage_and_channel.video.id, video_url).await?;
    Ok(())
}
