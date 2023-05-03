use crate::internals::{
    cloud::{models::payload::SrtPayload, traits::BucketClient},
    ServiceProvider,
};
use async_trait::async_trait;

#[async_trait]
pub trait SubtitlerClient<BC: BucketClient>: ServiceProvider {
    fn estimate_time(&self, payload: &SrtPayload, bucket_client: &BC) -> u32;
    async fn subtitle(
        &self,
        payload: &SrtPayload,
        bucket_client: &BC,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
