pub trait Client {
    fn create_signed_upload_url(
        &self,
        bucket_name: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>>;
    fn create_signed_download_url(
        &self,
        bucket_name: &str,
        expires_in: u16,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
