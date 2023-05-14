use crate::{
    database::models::video::VideoWithStorage,
    internals::{cloud::traits::BucketClient, ServiceProvider},
};
use async_trait::async_trait;

#[async_trait]
pub trait SubtitlerClient<BC: BucketClient>: ServiceProvider {
    fn estimate_time(&self, payload: &VideoWithStorage, bucket_client: &BC) -> u32;
    async fn subtitle(
        &self,
        payload: &VideoWithStorage,
        bucket_client: &BC,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Sync + Send>>;
}
