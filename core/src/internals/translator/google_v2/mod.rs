mod payload;

use payload::GoogleTranslateResponse;

use async_trait::async_trait;

use crate::internals::ServiceProvider;

use super::traits::TranslatorClient;

const MAX_SENTENCES_PER_REQUEST: usize = 128;

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
        _text: &str,
    ) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
        todo!("Implement GoogleTranslateV2Client::translate_sentence")
    }

    async fn translate_sentences(
        &self,
        sentences: Vec<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>> {
        let url = &self.api_base_url;
        let api_key = &self.api_key;

        let mut final_translated_sentences: Vec<String> = vec![];
        let mut i: usize = 0;
        let sentences_len = sentences.len();

        while i < sentences_len {
            let x = if i + MAX_SENTENCES_PER_REQUEST > sentences_len {
                sentences_len
            } else {
                i + MAX_SENTENCES_PER_REQUEST
            };

            let buff = &sentences[i..x];

            println!("Translating sentences {} to {}", i, x);

            let request_body = serde_json::json!({
                "q": buff,
                "source": "en-US",
                "target": "pt-BR"
            });

            let res = self
                .client
                .post(url)
                .query(&[("key", api_key)])
                .body(request_body.to_string())
                .send()
                .await?;

            let response_status = res.status();
            let text = res.text().await?;

            let response_body: GoogleTranslateResponse = match serde_json::from_str(&text) {
                Ok(response_body) => response_body,
                Err(e) => {
                    println!("status : {}", response_status);
                    println!("error : {}", e);
                    print!("");
                    println!("text {}", text);
                    Err(e)?
                }
            };

            let translated_sentences = response_body.data.translations;

            let translated_sentences: Vec<String> = translated_sentences
                .into_iter()
                .map(|translation| translation.translated_text)
                .collect();

            final_translated_sentences.extend(translated_sentences);

            i = x;
        }

        Ok(final_translated_sentences)
    }
}
