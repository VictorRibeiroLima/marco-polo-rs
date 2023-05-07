use async_trait::async_trait;
use futures::executor::block_on;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    GetObjectRequest, PutObjectRequest, S3,
};
use tokio::io::AsyncReadExt;

use crate::internals::{cloud::traits::BucketClient, ServiceProvider};

pub struct S3Client {
    region: rusoto_core::Region,
    credential: rusoto_credential::AwsCredentials,
    bucket_name: String,
    client: rusoto_s3::S3Client,
}

impl S3Client {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Creating S3 client...");
        let region = rusoto_core::Region::SaEast1;
        let bucket_name = std::env::var("AWS_BUCKET_NAME")?;

        let credential = block_on(EnvironmentProvider::default().credentials())?;
        let client = rusoto_s3::S3Client::new(region.clone());
        return Ok(Self {
            region,
            credential,
            bucket_name,
            client,
        });
    }
}

impl ServiceProvider for S3Client {
    fn id() -> i32 {
        2
    }
}

#[async_trait]
impl BucketClient for S3Client {
    async fn upload_file(
        &self,
        file_path: &str,
        file: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = PutObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_path.to_string(),
            body: Some(file.into()),
            ..Default::default()
        };

        self.client.put_object(request).await?;

        Ok(())
    }

    async fn create_signed_upload_url(
        &self,
        expiration: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let file_name = format!("videos/raw/{}.mkv", uuid);
        return self
            .create_signed_upload_url_with_uri(&file_name, expiration)
            .await;
    }

    async fn create_signed_upload_url_with_uri(
        &self,
        file_uri: &str,
        expiration: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = PutObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_uri.to_string(),
            ..Default::default()
        };

        let mut option = PreSignedRequestOption::default();
        option.expires_in = std::time::Duration::from_secs(expiration as u64);

        let url = request.get_presigned_url(&self.region, &self.credential, &option);
        return Ok(url);
    }

    async fn create_signed_download_url(
        &self,
        file_uri: &str,
        expiration: Option<u16>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let expiration = match expiration {
            Some(expiration) => std::time::Duration::from_secs(expiration as u64),
            None => std::time::Duration::from_secs(60 * 60 * 24 * 7),
        };
        let request = GetObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_uri.to_string(),
            ..Default::default()
        };

        let mut option = PreSignedRequestOption::default();
        option.expires_in = expiration;

        let url = request.get_presigned_url(&self.region, &self.credential, &option);
        return Ok(url);
    }

    async fn download_file(&self, file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let request = GetObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_path.to_string(),
            ..Default::default()
        };

        let response = self.client.get_object(request).await?;
        let body = match response.body {
            Some(body) => body,
            None => return Err("No body found".into()),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let mut reader = body.into_async_read();
        reader.read_to_end(&mut buffer).await?;

        Ok(buffer)
    }
}
