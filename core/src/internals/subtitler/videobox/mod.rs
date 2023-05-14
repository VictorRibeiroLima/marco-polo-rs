use std::println;

use async_trait::async_trait;
use serde_json::json;

use crate::{
    database::models::video::VideoWithStorage,
    internals::{cloud::aws::s3::S3Client, ServiceProvider},
};

use super::traits::SubtitlerClient;

use crate::internals::cloud::traits::BucketClient;

pub struct VideoBoxClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub api_token: String,
}

impl VideoBoxClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let base_url = std::env::var("VIDEO_BOX_BASE_URL").unwrap();
        let api_token = std::env::var("VIDEO_BOX_API_KEY").unwrap();

        return Self {
            client,
            base_url,
            api_token,
        };
    }
}

impl ServiceProvider for VideoBoxClient {
    fn id() -> i32 {
        return 5;
    }
}

#[async_trait]
impl SubtitlerClient<S3Client> for VideoBoxClient {
    fn estimate_time(&self, _payload: &VideoWithStorage, _bucket_client: &S3Client) -> u32 {
        return 200;
    }

    async fn subtitle(
        &self,
        video: &VideoWithStorage,
        bucket_client: &S3Client,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let file_name = format!("{}.{}", video.video.id, video.storage.format.to_string());

        let video_uri = format!("videos/raw/{}", file_name);
        let translated_srt_uri = format!("translations/{}.srt", video.video.id);

        let presigned_video_url = bucket_client
            .create_signed_download_url(&video_uri, None)
            .await?;

        let presigned_srt_url = bucket_client
            .create_signed_download_url(&translated_srt_uri, None)
            .await?;

        let api_url = format!("{}/tasks", self.base_url);
        let command = format!(
            "ffmpeg -i \"{}\" -vf subtitles=\"{}\" {}",
            presigned_video_url, presigned_srt_url, file_name
        );

        let storage_credentials = json!({
          "access_key": std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
          "secret_key": std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
          "bucket": std::env::var("AWS_BUCKET_NAME").unwrap(),
          "folder": "videos/processed",
        });

        let body = json!({
           "command": command,
           "storage_credentials": storage_credentials
        });
        println!("{:?}", body);

        let response = self
            .client
            .post(&api_url)
            .header("key", &self.api_token)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            println!("{:?}", response_text);

            let response_body = serde_json::from_str::<serde_json::Value>(&response_text)?;

            let task_id = response_body["id"].as_str();

            let task_id = match task_id {
                Some(task_id) => task_id,
                None => return Err("Failed to create task. No id was received by VideoBox".into()),
            };

            return Ok(Some(task_id.to_string()));
        }

        Err("Failed to create task".into())
    }
}
