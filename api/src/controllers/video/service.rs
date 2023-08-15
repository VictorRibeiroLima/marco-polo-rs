use marco_polo_rs_core::{
    database::queries::{self, video::CreateVideoDto},
    internals::cloud::{
        models::payload::{PayloadType, VideoDownloadPayload},
        traits::QueueClient,
    },
};
use sqlx::{types::Uuid, PgPool};

use super::dtos::create::CreateVideo;

pub async fn create_video(
    pool: &PgPool,
    body: &CreateVideo,
    queue_client: &impl QueueClient,
    user_id: i32,
    video_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let language = match &body.language {
        Some(language) => language,
        None => "en",
    };

    let start_time = match &body.start_time {
        Some(start_time) => start_time,
        None => "00:00:00",
    };

    let end_time: Option<&str> = match &body.end_time {
        Some(end_time) => Some(end_time),
        None => None,
    };

    let tags: Option<String> = match &body.tags {
        Some(tags) => {
            let tags = tags.join(";");
            Some(tags)
        }
        None => None,
    };

    let mut trx = pool.begin().await?;
    let original_video_id = queries::original_video::create(&mut *trx, &body.video_url).await?;

    queries::video::create(
        &mut *trx,
        CreateVideoDto {
            id: &video_id,
            user_id,
            title: &body.title,
            end_time,
            description: &body.description,
            channel_id: body.channel_id,
            language: &language,
            original_id: original_video_id,
            tags: tags.as_deref(),
            start_time,
        },
    )
    .await?;

    let payload: VideoDownloadPayload = VideoDownloadPayload {
        original_video_id,
        video_ids: vec![video_id],
    };

    queue_client
        .send_message(PayloadType::BatukaDownloadVideo(payload))
        .await?;

    trx.commit().await?;

    Ok(())
}
