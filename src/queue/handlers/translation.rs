use crate::{
    internals::{
        cloud::{
            models::payload::SrtPayload,
            traits::{CloudService, QueueClient},
        },
        subtitler::traits::SubtitlerClient,
        transcriber::traits::TranscriberClient,
        translator::traits::TranslatorClient,
    },
    queue::worker::Worker,
};

pub struct Handler<'a, CS, TC, TLC, SC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,

    SC: SubtitlerClient,
{
    worker: &'a Worker<CS, TC, TLC, SC>,
    message: &'a <<CS as CloudService>::QC as QueueClient>::M,
}

impl<'a, CS, TC, TLC, SC> Handler<'a, CS, TC, TLC, SC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,

    SC: SubtitlerClient,
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

        /*queue_client
        .change_message_visibility(&self.message, estimation as usize)
        .await?;*/

        subtitler_client.subtitle(payload, bucket_client).await?;

        return Ok(());
    }
}
