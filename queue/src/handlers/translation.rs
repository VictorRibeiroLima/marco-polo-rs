use std::sync::Arc;

use marco_polo_rs_core::{
    database::{
        models::video_storage::{StorageVideoStage, VideoFormat},
        queries::{self, storage::CreateStorageDto},
    },
    internals::{
        cloud::{
            models::payload::SrtPayload,
            traits::{CloudService, QueueClient},
        },
        subtitler::traits::SubtitlerClient,
        ServiceProvider,
    },
};

use crate::error::HandlerError;

pub struct Handler<'a, CS, SC>
where
    CS: CloudService,
    SC: SubtitlerClient<CS::BC>,
{
    cloud_service: &'a CS,
    subtitler_client: &'a SC,
    pool: Arc<sqlx::PgPool>,
    message: &'a <<CS as CloudService>::QC as QueueClient>::M,
}

impl<'a, CS, SC> Handler<'a, CS, SC>
where
    CS: CloudService,
    SC: SubtitlerClient<CS::BC>,
{
    pub fn new(
        cloud_service: &'a CS,
        subtitler_client: &'a SC,
        pool: Arc<sqlx::PgPool>,
        message: &'a <<CS as CloudService>::QC as QueueClient>::M,
    ) -> Self {
        Self {
            cloud_service,
            subtitler_client,
            pool,
            message,
        }
    }

    pub async fn handle(&self, payload: SrtPayload) -> Result<(), HandlerError> {
        let bucket_client = self.cloud_service.bucket_client();
        let queue_client = self.cloud_service.queue_client();

        let video = queries::video::find_by_id_with_storage(
            &self.pool,
            &payload.video_id,
            StorageVideoStage::Raw,
        )
        .await?;

        let estimation = self.subtitler_client.estimate_time(&video, bucket_client);

        queue_client
            .change_message_visibility(&self.message, estimation as usize)
            .await?;

        self.subtitler_client
            .subtitle(&video, bucket_client)
            .await?;

        let video_uri = format!(
            "videos/processed/{}.{}",
            payload.video_id,
            video.storage.format.to_string()
        );

        queries::storage::create(
            &self.pool,
            CreateStorageDto {
                format: VideoFormat::Mkv,
                storage_id: self.cloud_service.bucket_client().id(),
                video_id: &payload.video_id,
                video_uri: &video_uri,
                stage: StorageVideoStage::Processed,
            },
        )
        .await?;

        return Ok(());
    }
}
