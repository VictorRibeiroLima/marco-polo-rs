use async_trait::async_trait;

use crate::internals::ServiceProvider;

#[async_trait]
pub trait TranslatorClient: ServiceProvider {
    async fn translate(&self, text: &str) -> Result<String, Box<dyn std::error::Error>>;
}
