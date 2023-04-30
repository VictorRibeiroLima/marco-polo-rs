use futures::future::join_all;

use crate::{
    database::queries::{self, translation::CreateTranslationDto},
    internals::{
        cloud::{
            models::payload::SrtPayload,
            traits::{BucketClient, CloudService},
        },
        transcriber::traits::{Sentence, TranscriberClient},
        translator::traits::TranslatorClient,
    },
    queue::worker::Worker,
    util,
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

    pub async fn handle(&self, payload: SrtPayload) -> Result<(), Box<dyn std::error::Error>> {
        let worker = self.worker;
        let transcriber_client = &worker.transcriber_client;
        let bucket_client = &worker.cloud_service.bucket_client();
        let translator_id = TLC::id();
        let bucket_id = CS::id();

        let transcription =
            queries::transcription::find_by_video_id(&worker.pool, &payload.video_id).await?;

        let transcription_sentences = transcriber_client
            .get_transcription_sentences(&transcription.transcription_id)
            .await?;

        let (translation_raw, id) = self.translate(transcription_sentences).await?;

        let file_path = format!("srt_translations/{}.srt", payload.video_id);

        bucket_client
            .upload_file(&file_path, translation_raw.into())
            .await?;

        queries::translation::create(
            &worker.pool,
            CreateTranslationDto {
                video_id: &payload.video_id,
                translator_id,
                translation_id: id,
                storage_id: bucket_id,
                path: &file_path,
            },
        )
        .await?;

        Ok(())
    }

    pub async fn translate(
        &self,
        sentences: Vec<Sentence>,
    ) -> Result<(String, Option<String>), Box<dyn std::error::Error>>
    where
        CS: CloudService,
        TC: TranscriberClient,
    {
        let mut translation_futures = vec![];

        for sen in sentences {
            let translation = self.get_translated_sentence(sen);
            translation_futures.push(translation);
        }

        let resp = join_all(translation_futures).await;

        let mut translated_sentences = vec![];
        for sentence in resp {
            translated_sentences.push(sentence?);
        }

        let new_srt_buffer = util::srt::create_based_on_sentences(translated_sentences);

        Ok((new_srt_buffer, None))
    }

    async fn get_translated_sentence(
        &self,
        payload: Sentence,
    ) -> Result<Sentence, Box<dyn std::error::Error>>
    where
        TLC: TranslatorClient,
    {
        let worker = &self.worker;
        let translator_client = &worker.translator_client;

        let translation = translator_client.translate_sentence(payload.text).await?;
        let sentence = Sentence {
            text: translation,
            start_time: payload.start_time,
            end_time: payload.end_time,
        };
        Ok(sentence)
    }
}
