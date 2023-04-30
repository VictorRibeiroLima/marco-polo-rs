use async_trait::async_trait;

use crate::internals::ServiceProvider;

#[async_trait]
pub trait TranslatorClient: ServiceProvider {
    async fn translate_sentence(
        &self,
        sentence: String,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
