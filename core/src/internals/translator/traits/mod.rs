use async_trait::async_trait;

use crate::internals::ServiceProvider;

#[async_trait]
pub trait TranslatorClient: ServiceProvider {
    async fn translate_sentence(
        &self,
        sentence: &str,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>>;

    async fn translate_sentences(
        &self,
        sentences: Vec<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>>;
}
