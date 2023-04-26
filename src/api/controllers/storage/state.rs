use crate::infra;

pub struct StorageState<C>
where
    C: infra::traits::BucketClient,
{
    pub bucket_name: String,
    pub storage_client: C,
}

impl<C> StorageState<C>
where
    C: infra::traits::BucketClient,
{
    pub fn new(bucket_name: String, storage_client: C) -> Self {
        Self {
            bucket_name,
            storage_client,
        }
    }
}
