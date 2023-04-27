use crate::internals::ServiceProvider;

use self::s3::S3Client;

use super::traits::CloudService;

mod payload;
pub mod s3;
pub mod sqs;

pub struct AwsCloudService {
    pub bucket_client: S3Client,
    pub queue_client: sqs::SQSClient,
}

impl AwsCloudService {
    pub fn new(queue_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let bucket_client = S3Client::new()?;
        let queue_client = sqs::SQSClient::new(queue_url);

        return Ok(Self {
            bucket_client,
            queue_client,
        });
    }
}

impl ServiceProvider for AwsCloudService {
    fn id() -> u32 {
        return 1;
    }
}

impl CloudService for AwsCloudService {
    type BC = S3Client;
    type QC = sqs::SQSClient;

    fn bucket_client(&self) -> &Self::BC {
        &self.bucket_client
    }

    fn queue_client(&self) -> &Self::QC {
        &self.queue_client
    }
}
