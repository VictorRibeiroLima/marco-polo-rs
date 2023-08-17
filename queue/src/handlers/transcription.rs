use std::sync::Arc;

use marco_polo_rs_core::{
    database::{
        models::video::stage::VideoStage,
        queries::{self, translation::CreateTranslationDto},
    },
    internals::{
        cloud::{
            models::payload::SrtPayload,
            traits::{BucketClient, CloudService},
        },
        transcriber::traits::{Sentence, TranscriberClient},
        translator::traits::TranslatorClient,
        ServiceProvider,
    },
};

use marco_polo_rs_core::util::srt;

use crate::error::HandlerError;

pub struct Handler<'a, TC, CS, TLC>
where
    TC: TranscriberClient,
    CS: CloudService,
    TLC: TranslatorClient,
{
    transcriber_client: &'a TC,
    cloud_service: &'a CS,
    translator_client: &'a TLC,
    pool: Arc<sqlx::PgPool>,
}

impl<'a, TC, CS, TLC> Handler<'a, TC, CS, TLC>
where
    TC: TranscriberClient,
    CS: CloudService,
    TLC: TranslatorClient,
{
    pub fn new(
        transcriber_client: &'a TC,
        cloud_service: &'a CS,
        translator_client: &'a TLC,
        pool: Arc<sqlx::PgPool>,
    ) -> Handler<'a, TC, CS, TLC> {
        Self {
            transcriber_client,
            cloud_service,
            translator_client,
            pool,
        }
    }

    pub async fn handle(&self, payload: SrtPayload) -> Result<(), HandlerError> {
        let transcriber_client = self.transcriber_client;
        let bucket_client = self.cloud_service.bucket_client();
        let translator_id = self.translator_client.id();
        let bucket_id = bucket_client.id();

        let pool = self.pool.as_ref();

        queries::video::change_stage(pool, &payload.video_id, VideoStage::Translating).await?;

        let transcription =
            queries::transcription::find_by_video_id(&self.pool, &payload.video_id).await?;

        let transcription_sentences = transcriber_client
            .get_transcription_sentences(&transcription.transcription_id)
            .await?;

        let (translation_raw, id) = self.translate(transcription_sentences).await?;

        let file_path = format!("srt_translations/{}.srt", payload.video_id);

        bucket_client
            .upload_file(&file_path, translation_raw.into())
            .await?;

        queries::translation::create(
            &self.pool,
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
    ) -> Result<(String, Option<String>), Box<dyn std::error::Error + Sync + Send>> {
        let translated_sentences = self.get_translated_sentences(sentences).await?;

        let new_srt_buffer = srt::create_based_on_sentences(translated_sentences);

        Ok((new_srt_buffer, None))
    }

    async fn get_translated_sentences(
        &self,
        mut payload: Vec<Sentence>,
    ) -> Result<Vec<Sentence>, Box<dyn std::error::Error + Sync + Send>> {
        let translator_client = &self.translator_client;

        let mut texts_from_sentences: Vec<&str> = vec![];
        for sentence in payload.iter() {
            texts_from_sentences.push(&sentence.text);
        }

        let translations = translator_client
            .translate_sentences(texts_from_sentences)
            .await?;
        for (i, translation) in translations.into_iter().enumerate() {
            payload[i].text = translation.to_string();
        }

        Ok(payload)
    }
}
