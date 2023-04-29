use crate::{
    database::queries,
    internals::{
        cloud::{models::payload::SrtTranscriptionPayload, traits::CloudService},
        transcriber::traits::TranscriberClient,
    },
    queue::worker::Worker,
};

pub async fn handle<CS, TC>(
    worker: &Worker<CS, TC>,
    payload: SrtTranscriptionPayload,
) -> Result<(), Box<dyn std::error::Error>>
where
    CS: CloudService,
    TC: TranscriberClient,
{
    let transcriber_client = &worker.transcriber_client;
    let transcription =
        queries::transcription::find_by_video_id(&worker.pool, &payload.video_id).await?;

    let sentences = transcriber_client
        .get_transcription_sentences(&transcription.transcription_id)
        .await?;

    for sentence in sentences {
        println!("{:?}", sentence);
    }
    Ok(())
}
