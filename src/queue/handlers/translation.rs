use crate::{
    internals::{
        cloud::{
            models::payload::SrtPayload,
            traits::{CloudService, QueueClient},
        },
        transcriber::traits::TranscriberClient,
        translator::traits::TranslatorClient,
    },
    queue::worker::Worker,
};

pub struct Handler<'a, CS, TC, TLC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
{
    worker: &'a Worker<CS, TC, TLC>,
    message: &'a <<CS as CloudService>::QC as QueueClient>::M,
}

impl<'a, CS, TC, TLC> Handler<'a, CS, TC, TLC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
{
    pub fn new(
        worker: &'a Worker<CS, TC, TLC>,
        message: &'a <<CS as CloudService>::QC as QueueClient>::M,
    ) -> Self {
        Self { worker, message }
    }

    pub async fn handle(&self, payload: SrtPayload) -> Result<(), Box<dyn std::error::Error>> {
        let queue_client = self.worker.cloud_service.queue_client();
        println!("Handling translation");
        println!("Payload: {:?}", payload);
        queue_client
            .change_message_visibility(&self.message, 1000)
            .await?;

        return Ok(());
    }
}
