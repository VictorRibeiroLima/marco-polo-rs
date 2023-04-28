use sqlx::PgPool;

use crate::{
    api::models::error::AppError,
    database::queries::{self, video::UpdateVideoTranscriptionDto},
    internals::cloud::traits::BucketClient,
};

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
    let storage_id = C::id();

    let transcription_id = &req_body.transcript_id;

    println!("transcription_id: {}", transcription_id);

    let url = format!("{}/transcript/{}/srt", base_url, req_body.transcript_id);

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("Authorization", api_key)
        .send()
        .await?;

    let body = resp.text().await?;
    let body = body.as_bytes().to_vec();

    let video = queries::video::find_by_transcription_id(pool, transcription_id).await?;
    let file_name = format!("videos/{}/transcript.srt", video.id);

    bucket_client.upload_file(&file_name, body).await?;

    queries::video::update_transcription(
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
