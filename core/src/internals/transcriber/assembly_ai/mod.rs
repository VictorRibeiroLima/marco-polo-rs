use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Method};

use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{internals::ServiceProvider, SyncError};

use self::payload::{
    request::TranscribeRequestBody, response::TranscribeSentencesResponse, response::UploadResponse,
};

use super::traits::{Sentence, TranscriberClient};

mod payload;

use crate::util;

#[derive(Debug, Clone)]
pub struct AssemblyAiClient {
    api_key: String,
    api_url: String,
    webhook_url: String,
    webhook_token: String,
    client: Client,
}

impl AssemblyAiClient {
    pub fn new() -> Self {
        println!("Creating AssemblyAI client...");
        let api_key = std::env::var("ASSEMBLY_AI_API_KEY").unwrap();
        let api_url = std::env::var("ASSEMBLY_AI_BASE_URL").unwrap();

        let our_base_url = std::env::var("API_URL").unwrap();
        let endpoint_url = std::env::var("ASSEMBLY_AI_WEBHOOK_ENDPOINT").unwrap();

        let webhook_url = format!("{}/{}", our_base_url, endpoint_url);
        let webhook_token = std::env::var("ASSEMBLY_AI_WEBHOOK_TOKEN").unwrap();

        let client = Client::new();
        Self {
            api_key,
            api_url,
            webhook_url,
            webhook_token,
            client,
        }
    }
}

impl ServiceProvider for AssemblyAiClient {
    fn id(&self) -> i32 {
        return 3;
    }
}

#[async_trait]
impl TranscriberClient for AssemblyAiClient {
    async fn get_transcription_sentences(
        &self,
        transcription_id: &str,
    ) -> Result<Vec<Sentence>, SyncError> {
        let url = format!("{}/transcript/{}/sentences", self.api_url, transcription_id);

        let resp = self
            .client
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

    async fn transcribe(&self, media_url: &str) -> Result<String, SyncError> {
        let url = format!("{}/transcript", self.api_url);

        let client = Client::new();

        let req_body = TranscribeRequestBody {
            audio_url: media_url.to_string(),
            webhook_url: self.webhook_url.to_string(),
            webhook_auth_header_name: "Authorization".to_string(),
            webhook_auth_header_value: self.webhook_token.to_string(),
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

    async fn transcribe_from_file(&self, file_path: &str) -> Result<String, SyncError> {
        let path = PathBuf::from(file_path);
        let audio_buff = util::ffmpeg::extract_audio_from_video_to_buff(&path)?;

        let client = reqwest::Client::new();

        let api_url = format!("{}/upload", self.api_url);

        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_str(&self.api_key)?);
        headers.insert(
            "content-type",
            HeaderValue::from_static("application/octet-stream"),
        );

        let request = client
            .request(Method::POST, &api_url)
            .headers(headers)
            .body(audio_buff)
            .build()?;

        let response = client.execute(request).await?;

        let upload: UploadResponse = response.json().await?;

        let req_body = json!({
            "audio_url": upload.upload_url,
        });

        let parsed_body = serde_json::to_string(&req_body)?;

        let url = format!("{}/transcript", self.api_url);

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
            None => return Err("Could not get transcript id".into()),
        };

        return transcript_id;
    }

    async fn pool(&self, transcription_id: &str) -> Result<(), SyncError> {
        let client = reqwest::Client::new();
        let pooling_url = format!("{}/transcript/{}", self.api_url, transcription_id);
        loop {
            let pooling_resp = client
                .get(&pooling_url)
                .header("Authorization", &self.api_key)
                .send()
                .await?;

            let pooling_resp_body = pooling_resp.text().await?;

            let pooling_resp_body: Value = serde_json::from_str(&pooling_resp_body)?;

            let status = match pooling_resp_body["status"].as_str() {
                Some(status) => status,
                None => return Err("Could not get transcript status".into()),
            };

            if status == "completed" {
                return Ok(());
            } else if status == "error" {
                eprintln!(
                    "Transcription failed with error: {}",
                    pooling_resp_body["error"]
                );
                return Err("Transcription failed".into());
            }
            thread::sleep(Duration::from_secs(3));
        }
    }
}
