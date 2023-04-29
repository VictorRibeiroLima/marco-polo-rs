use crate::{
    database::queries,
    internals::{
        cloud::{models::payload::SrtTranscriptionPayload, traits::CloudService},
        transcriber::traits::{TranscriberClient, TranscriptionSentence},
        translator::{deepl::DeeplClient, traits::TranslatorClient},
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
}

impl<'a, CS, TC, TLC> Handler<'a, CS, TC, TLC>
where
    CS: CloudService,
    TC: TranscriberClient,
    TLC: TranslatorClient,
{
    pub fn new(worker: &'a Worker<CS, TC, TLC>) -> Handler<'a, CS, TC, TLC> {
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

        let translator_client = DeeplClient::new();

        let mut translation_futures = vec![];

        for sen in sentences {
            let translation = translator_client.translate(&sen.text);

            translation_futures.push(translation);
        }

        Ok(())
    }
}
