mod payload;

use payload::DeeplResponse;

use async_trait::async_trait;

use crate::internals::ServiceProvider;

use super::traits::TranslatorClient;

#[derive(Debug, Clone)]
pub struct DeeplClient {
    api_key: String,
    api_base_url: String,
    client: reqwest::Client,
}

impl DeeplClient {
    pub fn new() -> Self {
        println!("Creating Deepl client...");
        let api_key = std::env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");

        let api_key = format!("DeepL-Auth-Key {}", api_key);

        let api_base_url = std::env::var("DEEPL_BASE_URL").expect("DEEPL_BASE_URL not set");

        let client = reqwest::Client::new();

        DeeplClient {
            api_key,
            api_base_url,
            client,
        }
    }
}

impl ServiceProvider for DeeplClient {
    fn id(&self) -> i32 {
        return 4;
    }
}

#[async_trait]
impl TranslatorClient for DeeplClient {
    async fn translate_sentence(
        &self,
        text: &str,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        let text = text;
        let url = &self.api_base_url;

        let params = [
            ("source_lang", "EN"),
            ("target_lang", "PT-BR"),
            ("split_sentences", "0"),
            ("text", text),
        ];

        let res = self
            .client
            .post(url)
            .header("Authorization", &self.api_key)
            .form(&params)
            .send()
            .await?;

        let response_status = res.status();
        let text = res.text().await?;

        let response_body: DeeplResponse = match serde_json::from_str(&text) {
            Ok(response_body) => response_body,
            Err(e) => {
                println!("status : {}", response_status);
                println!("error : {}", e);
                print!("");
                println!("text {}", text);
                Err(e)?
            }
        };

        let translation = response_body.translations.first();

        let translation = match translation {
            Some(translation) => translation.text.to_string(),
            None => Err("No translation found")?,
        };

        Ok(translation)
    }

    async fn translate_sentences(
        &self,
        sentences: Vec<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
        todo!("Implement DeeplClient::translate_sentences")
    }
}
