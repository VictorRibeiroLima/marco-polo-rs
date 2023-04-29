use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

use crate::internals::ServiceProvider;

use self::payload::{request::TranscribeRequestBody, response::TranscribeSentencesResponse};

use super::traits::{TranscriberClient, TranscriptionSentence};

mod payload;

pub struct AssemblyAiClient {
    api_key: String,
    api_url: String,
    webhook_url: String,
}

impl AssemblyAiClient {
    pub fn new() -> Self {
        let api_key = std::env::var("ASSEMBLY_AI_API_KEY").unwrap();
        let api_url = std::env::var("ASSEMBLY_AI_BASE_URL").unwrap();

        let our_base_url = std::env::var("API_URL").unwrap();
        let endpoint_url = std::env::var("ASSEMBLY_AI_WEBHOOK_ENDPOINT").unwrap();
        let webhook_url = format!("{}/{}", our_base_url, endpoint_url);
        Self {
            api_key,
            api_url,
            webhook_url,
        }
    }
}

impl ServiceProvider for AssemblyAiClient {
    fn id() -> i32 {
        return 2;
    }
}

#[async_trait]
impl TranscriberClient for AssemblyAiClient {
    async fn get_transcription_sentences(
        &self,
        transcription_id: &str,
    ) -> Result<Vec<TranscriptionSentence>, Box<dyn std::error::Error>> {
        let url = format!("{}/transcript/{}/sentences", self.api_url, transcription_id);
        let client = Client::new();

        let resp = client
            .get(&url)
            .header("Authorization", self.api_key.to_string())
            .send()
            .await?;

        let resp_body = resp.text().await?;

        let resp_body: TranscribeSentencesResponse = serde_json::from_str(&resp_body)?;

        let sentences = resp_body
            .sentences
            .into_iter()
            .map(|sentence| sentence.into())
            .collect();

        return Ok(sentences);
    }

    async fn transcribe(&self, media_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/transcript", self.api_url);
        let client = Client::new();

        let req_body = TranscribeRequestBody {
            audio_url: media_url.to_string(),
            webhook_url: self.webhook_url.to_string(),
            webhook_auth_header_name: "Authorization".to_string(),
            webhook_auth_header_value: "Corn Incident".to_string(),
        };

        let parsed_body = serde_json::to_string(&req_body)?;

        let res = client
            .post(&url)
            .header("Authorization", self.api_key.to_string())
            .body(parsed_body)
            .send()
            .await?;

        let res_body = res.text().await?;

        let res_body: Value = serde_json::from_str(&res_body)?;

        let transcript_id = match res_body["id"].as_str() {
            Some(id) => Ok(id.to_string()),
            None => {
                return Err("Could not get transcript id".into());
            }
        };

        return transcript_id;
    }
}
