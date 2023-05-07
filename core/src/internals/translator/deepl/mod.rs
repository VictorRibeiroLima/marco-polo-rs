mod payload;

use payload::DeeplResponse;

use async_trait::async_trait;

use crate::internals::ServiceProvider;

use super::traits::TranslatorClient;

pub struct DeeplClient {
    api_key: String,
    api_base_url: String,
}

impl DeeplClient {
    pub fn new() -> Self {
        println!("Creating Deepl client...");
        let api_key = std::env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");

        let api_key = format!("DeepL-Auth-Key {}", api_key);

        let api_base_url = std::env::var("DEEPL_BASE_URL").expect("DEEPL_BASE_URL not set");

        DeeplClient {
            api_key,
            api_base_url,
        }
    }
}

impl ServiceProvider for DeeplClient {
    fn id() -> i32 {
        return 4;
    }
}

#[async_trait]
impl TranslatorClient for DeeplClient {
    async fn translate_sentence(&self, text: String) -> Result<String, Box<dyn std::error::Error>> {
        let text = &*text;
        let url = &self.api_base_url;

        let params = [
            ("source_lang", "EN"),
            ("target_lang", "PT-BR"),
            ("split_sentences", "0"),
            ("text", text),
        ];
        let client = reqwest::Client::new();

        let res = client
            .post(url)
            .header("Authorization", &self.api_key)
            .form(&params)
            .send()
            .await?;

        let text = res.text().await?;

        let response_body: DeeplResponse = serde_json::from_str(&text)?;

        let translation = response_body.translations.first();

        let translation = match translation {
            Some(translation) => translation.text.to_string(),
            None => Err("No translation found")?,
        };

        Ok(translation)
    }
}
