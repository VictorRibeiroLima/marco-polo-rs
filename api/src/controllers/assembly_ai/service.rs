use sqlx::PgPool;

use marco_polo_rs_core::{
    database::queries::{self, transcription::UpdateVideoTranscriptionDto, video::CreateErrorsDto},
    internals::cloud::traits::BucketClient,
};
use uuid::Uuid;

use crate::models::error::AppError;

use super::models::WebhookRequestBody;

pub async fn webhook<C>(
    req_body: WebhookRequestBody,
    pool: &PgPool,
    bucket_client: &C,
) -> Result<(), AppError>
where
    C: BucketClient,
{
    let api_key = std::env::var("ASSEMBLY_AI_API_KEY").unwrap();
    let base_url = std::env::var("ASSEMBLY_AI_BASE_URL").unwrap();
    let storage_id = bucket_client.id();

    let transcription_id = &req_body.transcript_id;

    let video = queries::video::find_by_transcription_id(pool, transcription_id).await?;

    let url = format!("{}/transcript/{}/srt", base_url, req_body.transcript_id);

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("Authorization", api_key)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        transcription_error(pool, video.id, &format!("AssemblyAI error: {}", body)).await?;
        return Ok(());
    }

    let body = body.as_bytes().to_vec();

    let file_name = format!("srt_transcriptions/{}.srt", video.id);

    bucket_client.upload_file(&file_name, body).await?;

    queries::transcription::update(
        pool,
        UpdateVideoTranscriptionDto {
            video_id: video.id,
            storage_id,
            path: file_name,
        },
    )
    .await?;

    return Ok(());
}

async fn transcription_error(pool: &PgPool, video_id: Uuid, error: &str) -> Result<(), AppError> {
    let dto: CreateErrorsDto = CreateErrorsDto {
        video_ids: vec![video_id],
        error,
    };
    queries::video::create_errors(&pool, dto).await?;
    Ok(())
}
