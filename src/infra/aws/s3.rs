use async_std::task::block_on;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    PutObjectRequest,
};

use crate::infra::traits::BucketClient;

pub struct S3Client {
    region: rusoto_core::Region,
    credential: rusoto_credential::AwsCredentials,
}

impl S3Client {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let region = rusoto_core::Region::SaEast1;

        let credential = block_on(EnvironmentProvider::default().credentials())?;
        Ok(Self { region, credential })
    }
}

impl BucketClient for S3Client {
    fn create_signed_upload_url(
        &self,
        bucket_name: &str,
        expiration: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let key = uuid::Uuid::new_v4().to_string();
        let request = PutObjectRequest {
            bucket: bucket_name.to_string(),
            key,
            ..Default::default()
        };

        let mut option = PreSignedRequestOption::default();
        option.expires_in = std::time::Duration::from_secs(expiration as u64);

        let url = request.get_presigned_url(&self.region, &self.credential, &option);
        return Ok(url);
    }

    fn create_signed_download_url(
        &self,
        bucket_name: &str,
        _expiration: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "S3 signed download url for bucket: {}",
            bucket_name
        ))
    }
}
