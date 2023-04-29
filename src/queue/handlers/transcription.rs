use futures::future::join_all;

use crate::{
    database::queries,
    internals::{
        cloud::{models::payload::SrtTranscriptionPayload, traits::CloudService},
        transcriber::traits::{TranscriberClient, TranscriptionSentence},
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

    /*
    {}
    {} --> {}
    {}
     */
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

        let mut translation_futures = vec![];

        for sen in sentences {
            let translation = self.send_translation(sen);
            translation_futures.push(translation);
        }

        let resp = join_all(translation_futures).await;

        for (i, result) in resp.into_iter().enumerate() {
            let translation = result?;
            println!("{}: {}", i, translation.text);
        }
        Ok(())
    }

    async fn send_translation(
        &self,
        payload: TranscriptionSentence,
    ) -> Result<TranscriptionSentence, Box<dyn std::error::Error>>
    where
        TLC: TranslatorClient,
    {
        let worker = &self.worker;
        let translator_client = &worker.translator_client;

        let translation = translator_client.translate(payload.text).await?;
        let sentence = TranscriptionSentence {
            text: translation,
            start_time: payload.start_time,
            end_time: payload.end_time,
        };
        Ok(sentence)
    }
}
