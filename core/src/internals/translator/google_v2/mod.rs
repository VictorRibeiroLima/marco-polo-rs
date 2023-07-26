mod payload;

use payload::GoogleTranslateResponse;

use async_trait::async_trait;

use crate::internals::ServiceProvider;

use super::traits::TranslatorClient;

#[derive(Debug, Clone)]
pub struct GoogleTranslateV2Client {
    api_key: String,
    api_base_url: String,
    client: reqwest::Client,
}

impl GoogleTranslateV2Client {
    pub fn new() -> Self {
        println!("Creating Google Translator V2 client...");
        let api_key =
            std::env::var("GOOGLE_TRANSLATE_API_KEY").expect("GOOGLE_TRANSLATE_API_KEY not set");

        let api_base_url = std::env::var("GOOGLE_TRANSLATE_API_BASE_URL")
            .expect("GOOGLE_TRANSLATE_API_BASE_URL not set");

        let client = reqwest::Client::new();

        GoogleTranslateV2Client {
            api_key,
            api_base_url,
            client,
        }
    }
}

impl ServiceProvider for GoogleTranslateV2Client {
    fn id(&self) -> i32 {
        return 6;
    }
}

#[async_trait]
impl TranslatorClient for GoogleTranslateV2Client {
    async fn translate_sentence(
        &self,
        text: &str,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        todo!("Implement GoogleTranslateV2Client::translate_sentence")
    }

    async fn translate_sentences(
        &self,
        sentences: Vec<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
    }
}
