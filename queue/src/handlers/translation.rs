use marco_polo_rs_core::{
    database::{
        models::video_storage::{VideoFormat, VideoStage},
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

use crate::{worker::Worker, CloudServiceInUse, Message};

pub struct Handler<'a> {
    worker: &'a Worker,
    message: &'a Message,
}

impl<'a> Handler<'a> {
    pub fn new(worker: &'a Worker, message: &'a Message) -> Self {
        Self { worker, message }
    }

    pub async fn handle(
        &self,
        payload: SrtPayload,
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let subtitler_client = &self.worker.subtitler_client;
        let bucket_client = self.worker.cloud_service.bucket_client();
        let queue_client = self.worker.cloud_service.queue_client();

        let video = queries::video::find_by_id_with_storage(
            &self.worker.pool,
            &payload.video_id,
            VideoStage::Raw,
        )
        .await?;

        let estimation = subtitler_client.estimate_time(&video, bucket_client);

        queue_client
            .change_message_visibility(&self.message, estimation as usize)
            .await?;

        subtitler_client.subtitle(&video, bucket_client).await?;

        let video_uri = format!("videos/processed/{}.{}", payload.video_id, "mkv");

        queries::storage::create(
            &self.worker.pool,
            CreateStorageDto {
                format: VideoFormat::Mkv,
                storage_id: CloudServiceInUse::id(),
                video_id: &payload.video_id,
                video_uri: &video_uri,
                stage: VideoStage::Processed,
            },
        )
        .await?;

        return Ok(());
    }
}
