use super::traits::Client;

pub struct S3Client {}

impl S3Client {
    pub fn new() -> Self {
        Self {}
    }
}

impl Client for S3Client {
    fn create_signed_upload_url(
        &self,
        bucket_name: &str,
        _expiration: u16,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("S3 signed upload url for bucket: {}", bucket_name))
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
