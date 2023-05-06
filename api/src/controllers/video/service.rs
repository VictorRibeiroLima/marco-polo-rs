use marco_polo_rs_core::{
    database::{
        models::video_storage::{VideoFormat, VideoStage},
        queries::{self, storage::CreateStorageDto, video::CreateVideoDto},
    },
    internals::cloud::traits::BucketClient,
};
use sqlx::{types::Uuid, PgPool};

use super::dtos::create::CreateVideo;

pub async fn create_video<BC: BucketClient>(
    pool: &PgPool,
    body: CreateVideo,
    bucket_client: &BC,
    user_id: i32,
    video_id: Uuid,
    file: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (format, format_extension) = match body.format {
        Some(format) => {
            let format_extension = (&format).to_string();
            (format, format_extension)
        }
        None => (VideoFormat::Mkv, "mkv".into()),
    };

    let language = match body.language {
        Some(language) => language,
        None => "en".into(),
    };

    let file_uri = format!("videos/raw/{}.{}", video_id, format_extension);

    bucket_client.upload_file(&file_uri, file).await?;

    queries::video::create(
        pool,
        CreateVideoDto {
            id: &video_id,
            user_id,
            title: &body.title,
            description: &body.description,
            channel_id: body.channel_id,
            language: &language,
        },
    )
    .await?;

    queries::storage::create(
        &pool,
        CreateStorageDto {
            video_id: &video_id,
            video_uri: &file_uri,
            storage_id: BC::id(),
            format: format.clone(),
            stage: VideoStage::Raw,
        },
    )
    .await?;

    Ok(())
}
