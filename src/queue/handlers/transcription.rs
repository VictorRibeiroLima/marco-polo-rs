use crate::{
    database::queries,
    internals::{
        cloud::{models::payload::SrtTranscriptionPayload, traits::CloudService},
        transcriber::traits::{TranscriberClient, TranscriptionSentence},
    },
    queue::worker::Worker,
};

pub struct Handler<'a, CS, TC>
where
    CS: CloudService,
    TC: TranscriberClient,
{
    worker: &'a Worker<CS, TC>,
}

impl<'a, CS, TC> Handler<'a, CS, TC>
where
    CS: CloudService,
    TC: TranscriberClient,
{
    pub fn new(worker: &'a Worker<CS, TC>) -> Handler<'a, CS, TC> {
        Self { worker }
    }

    pub async fn get_sentences(
        &self,
        payload: SrtTranscriptionPayload,
    ) -> Result<Vec<TranscriptionSentence>, Box<dyn std::error::Error>>
    where
        CS: CloudService,
        TC: TranscriberClient,
    {
        let worker = &self.worker;
        let transcriber_client = &worker.transcriber_client;
        let transcription =
            queries::transcription::find_by_video_id(&worker.pool, &payload.video_id).await?;

        let sentences = transcriber_client
            .get_transcription_sentences(&transcription.transcription_id)
            .await?;

        Ok(sentences)
    }

    pub async fn translate(
        &self,
        sentences: Vec<TranscriptionSentence>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        CS: CloudService,
        TC: TranscriberClient,
    {
        //let worker = &self.worker;
        //let transcriber_client = &worker.transcriber_client;

        for sen in sentences {
            println!("Sentence: {}", sen.text);
        }
        Ok(())
    }
}
