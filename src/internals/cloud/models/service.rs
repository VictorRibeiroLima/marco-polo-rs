use crate::internals::cloud::traits::{BucketClient, QueueClient};

pub trait CloudService {
    type BC: BucketClient;
    type QC: QueueClient;
    fn bucket_client(&self) -> &Self::BC;
    fn queue_client(&self) -> &Self::QC;
}
