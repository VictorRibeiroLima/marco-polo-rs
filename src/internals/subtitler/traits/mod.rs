use crate::internals::{
    cloud::{models::payload::SrtPayload, traits::BucketClient},
    ServiceProvider,
};
use async_trait::async_trait;

#[async_trait]
pub trait SubtitlerClient: ServiceProvider {
    fn estimate_time<BC: BucketClient>(&self, payload: &SrtPayload, bucket_client: &BC) -> u32;
    async fn subtitle<BC: BucketClient>(
        &self,
        payload: SrtPayload,
        bucket_client: &BC,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
