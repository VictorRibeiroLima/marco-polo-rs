use std::{fs::File, io::Read};

use async_trait::async_trait;
use futures::executor::block_on;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    CompletedPart, CreateMultipartUploadRequest, GetObjectRequest, PutObjectRequest,
    UploadPartRequest, S3,
};
use tokio::io::AsyncReadExt;

use crate::{
    internals::{cloud::traits::BucketClient, ServiceProvider},
    SyncError,
};

#[derive(Clone)]
pub struct S3Client {
    region: rusoto_core::Region,
    credential: rusoto_credential::AwsCredentials,
    bucket_name: String,
    client: rusoto_s3::S3Client,
}

impl S3Client {
    pub fn new() -> Result<Self, SyncError> {
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
    fn id(&self) -> i32 {
        2
    }
}

#[async_trait]
impl BucketClient for S3Client {
    async fn upload_file(&self, file_uri: &str, file: Vec<u8>) -> Result<(), SyncError> {
        let request = PutObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_uri.to_string(),
            body: Some(file.into()),
            ..Default::default()
        };

        self.client.put_object(request).await?;

        Ok(())
    }

    async fn upload_file_from_path(
        &self,
        file_uri: &str,
        file_path: &str,
    ) -> Result<(), SyncError> {
        const PART_SIZE: usize = 5 * 1024 * 1024; // 5MB

        // Create a new multipart upload
        let create_request = CreateMultipartUploadRequest {
            bucket: self.bucket_name.clone(),
            key: file_uri.to_string(),
            ..Default::default()
        };
        let create_response = self.client.create_multipart_upload(create_request).await?;
        let upload_id = create_response.upload_id.unwrap();

        // Open the file
        let file = File::open(file_path)?;
        let mut reader = std::io::BufReader::new(file);

        let mut part_number = 1;
        let mut completed_parts = Vec::new();

        // Read and upload parts until the end of the file
        loop {
            let mut part_buffer = Vec::with_capacity(PART_SIZE);
            let bytes_read = reader
                .by_ref()
                .take(PART_SIZE as u64)
                .read_to_end(&mut part_buffer)?;

            if bytes_read == 0 {
                // End of file reached
                break;
            }

            let upload_request = UploadPartRequest {
                bucket: self.bucket_name.clone(),
                key: file_uri.to_string(),
                upload_id: upload_id.clone(),
                part_number: part_number,
                body: Some(part_buffer.into()),
                ..Default::default()
            };

            let upload_response = self.client.upload_part(upload_request).await?;
            let completed_part = CompletedPart {
                e_tag: upload_response.e_tag,
                part_number: Some(part_number),
            };

            completed_parts.push(completed_part);

            part_number += 1;
        }

        // Complete the multipart upload
        let complete_request = rusoto_s3::CompleteMultipartUploadRequest {
            bucket: self.bucket_name.clone(),
            key: file_uri.to_string(),
            upload_id: upload_id.clone(),
            multipart_upload: Some(rusoto_s3::CompletedMultipartUpload {
                parts: Some(completed_parts),
                ..Default::default()
            }),
            ..Default::default()
        };

        self.client
            .complete_multipart_upload(complete_request)
            .await?;

        println!("File uploaded successfully");

        Ok(())
    }

    async fn create_signed_upload_url(&self, expiration: u16) -> Result<String, SyncError> {
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
    ) -> Result<String, SyncError> {
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
    ) -> Result<String, SyncError> {
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

    async fn download_file(&self, file_path: &str) -> Result<Vec<u8>, SyncError> {
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

    async fn download_file_to_path(
        &self,
        file_path: &str,
        destination_path: &str,
    ) -> Result<(), SyncError> {
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

        let mut reader = body.into_async_read();
        let mut file = tokio::fs::File::create(destination_path).await?;
        tokio::io::copy(&mut reader, &mut file).await?;

        Ok(())
    }
}
