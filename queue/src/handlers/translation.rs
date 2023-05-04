use marco_polo_rs_core::{
    database::{
        models::video_storage::{VideoFormat, VideoStage},
        queries::{self, video::CreateStorageDto},
    },
    internals::{
        cloud::{
            models::payload::SrtPayload,
            traits::{CloudService, QueueClient},
        },
        subtitler::traits::SubtitlerClient,
        transcriber::traits::TranscriberClient,
        translator::traits::TranslatorClient,
    },
};

use crate::worker::Worker;

pub struct Handler<'a, CS, TC, TLC, SC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,

    SC: SubtitlerClient<CS::BC>,
{
    worker: &'a Worker<CS, TC, TLC, SC>,
    message: &'a <<CS as CloudService>::QC as QueueClient>::M,
}

impl<'a, CS, TC, TLC, SC> Handler<'a, CS, TC, TLC, SC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
    SC: SubtitlerClient<CS::BC>,
{
    pub fn new(
        worker: &'a Worker<CS, TC, TLC, SC>,
        message: &'a <<CS as CloudService>::QC as QueueClient>::M,
    ) -> Self {
        Self { worker, message }
    }

    pub async fn handle(&self, payload: SrtPayload) -> Result<(), Box<dyn std::error::Error>> {
        let subtitler_client = &self.worker.subtitler_client;
        let bucket_client = self.worker.cloud_service.bucket_client();
        let queue_client = self.worker.cloud_service.queue_client();

        let estimation = subtitler_client.estimate_time(&payload, bucket_client);

        queue_client
            .change_message_visibility(&self.message, estimation as usize)
            .await?;

        subtitler_client.subtitle(&payload, bucket_client).await?;

        let video_uri = format!("videos/processed/{}.{}", payload.video_id, "mkv");

        queries::video::create_storage(
            &self.worker.pool,
            CreateStorageDto {
                format: VideoFormat::Mkv,
                storage_id: CS::id(),
                video_id: &payload.video_id,
                video_uri: &video_uri,
                stage: VideoStage::Processed,
            },
        )
        .await?;

        return Ok(());
    }
}
