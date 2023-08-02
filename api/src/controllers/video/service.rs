use marco_polo_rs_core::database::queries::{self, video::CreateVideoDto};
use sqlx::{types::Uuid, PgPool};

use super::dtos::create::CreateVideo;

pub async fn create_video(
    pool: &PgPool,
    body: &CreateVideo,
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

    let tags: Option<String> = match &body.tags {
        Some(tags) => {
            let tags = tags.join(";");
            Some(tags)
        }
        None => None,
    };

    queries::video::create(
        pool,
        CreateVideoDto {
            id: &video_id,
            user_id,
            title: &body.title,
            description: &body.description,
            channel_id: body.channel_id,
            language: &language,
            original_url: &body.video_url,
            tags: tags.as_deref(),
            start_time,
        },
    )
    .await?;

    Ok(())
}
