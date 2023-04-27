use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::traits::TranscriberClient;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Root {
    #[serde(rename = "audio_url")]
    pub audio_url: String,
    #[serde(rename = "webhook_url")]
    pub webhook_url: String,
    #[serde(rename = "webhook_auth_header_name")]
    pub webhook_auth_header_name: String,
    #[serde(rename = "webhook_auth_header_value")]
    pub webhook_auth_header_value: String,
}

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

#[async_trait]
impl TranscriberClient for AssemblyAiClient {
    async fn transcribe(&self, media_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/transcript", self.api_url);
        let client = Client::new();

        let req_body = Root {
            audio_url: media_url.to_string(),
            webhook_url: self.webhook_url.to_string(),
            webhook_auth_header_name: "Authorization".to_string(),
            webhook_auth_header_value: "Corn Incident".to_string(),
        };

        println!("OI");
        println!("{:?}", url);
        let parsed_body = serde_json::to_string(&req_body)?;

        println!("OI2");
        let res = client
            .post(&url)
            .header("Authorization", self.api_key.to_string())
            .body(parsed_body)
            .send()
            .await?;

        println!("OI3");
        let res_body = res.text().await?;

        println!("{:?}", res_body);
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
