use async_trait::async_trait;

use crate::internals::ServiceProvider;

#[async_trait]
pub trait TranscriberClient: ServiceProvider {
    async fn transcribe(&self, media_url: &str) -> Result<String, Box<dyn std::error::Error>>;
}
