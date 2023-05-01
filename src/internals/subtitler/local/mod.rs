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

    async fn subtitle<BC: BucketClient>(
        &self,
        payload: SrtPayload,
        bucket_client: &BC,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
