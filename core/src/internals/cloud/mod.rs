use self::aws::AwsCloudService;

pub mod aws;
pub mod models;
pub mod traits;

#[cfg(not(release))]
pub mod test;

pub fn default_cloud_service() -> AwsCloudService {
    let queue_url = std::env::var("AWS_QUEUE_URL").unwrap();
    let cloud_service = aws::AwsCloudService::new(queue_url).unwrap();
    return cloud_service;
}
