use std::borrow::Borrow;

use async_trait::async_trait;

use crate::internals::ServiceProvider;

#[async_trait]
pub trait TranslatorClient: ServiceProvider {
    async fn translate<T: Borrow<String>>(
        &self,
        text: T,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
