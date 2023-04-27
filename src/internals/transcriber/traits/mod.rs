use async_trait::async_trait;

#[async_trait]
pub trait TranscriberClient {
    async fn transcribe(&self, media_url: &str) -> Result<String, Box<dyn std::error::Error>>;
}
