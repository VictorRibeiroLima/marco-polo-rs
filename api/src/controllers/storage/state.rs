use marco_polo_rs_core::internals::cloud;

pub struct StorageState<C>
where
    C: cloud::traits::BucketClient,
{
    pub storage_client: C,
}

impl<C> StorageState<C>
where
    C: cloud::traits::BucketClient,
{
    pub fn new(storage_client: C) -> Self {
        Self { storage_client }
    }
}
