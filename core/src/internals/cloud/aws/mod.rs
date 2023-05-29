use crate::{internals::ServiceProvider, SyncError};

use self::s3::S3Client;

use super::traits::CloudService;

mod payload;
pub mod s3;
pub mod sqs;

#[derive(Clone)]
pub struct AwsCloudService {
    pub bucket_client: S3Client,
    pub queue_client: sqs::SQSClient,
}

impl AwsCloudService {
    pub fn new(queue_url: String) -> Result<Self, SyncError> {
        let bucket_client = S3Client::new()?;
        let queue_client = sqs::SQSClient::new(queue_url);

        return Ok(Self {
            bucket_client,
            queue_client,
        });
    }
}

impl ServiceProvider for AwsCloudService {
    fn id(&self) -> i32 {
        return 2;
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
