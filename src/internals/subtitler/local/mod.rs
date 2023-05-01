use crate::internals::{
    cloud::{models::payload::SrtPayload, traits::BucketClient},
    ServiceProvider,
};

use super::traits::SubtitlerClient;
use async_trait::async_trait;

pub struct LocalClient;

impl ServiceProvider for LocalClient {
    fn id() -> i32 {
        1
    }
}

#[async_trait]
impl SubtitlerClient for LocalClient {
    fn estimate_time<BC: BucketClient>(&self, _: &SrtPayload, _: &BC) -> u32 {
        10000
    }

    async fn subtitle<BC: BucketClient + Sync>(
        &self,
        payload: SrtPayload,
        bucket_client: &BC,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let video_id = payload.video_id.to_string();
        let video_uri = format!("videos/raw/{}.{}", video_id, ".mp4"); // for now, we only support mp4,refactor later
        let _video = bucket_client.download_file(&video_uri).await?;
        let _srt = bucket_client.download_file(&payload.srt_uri).await?;
        Ok(())
    }
}
