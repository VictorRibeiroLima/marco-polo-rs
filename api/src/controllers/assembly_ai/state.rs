use marco_polo_rs_core::internals::cloud::traits::BucketClient;

pub struct AssemblyAiState<C>
where
    C: BucketClient,
{
    pub storage_client: C,
}

impl<C> AssemblyAiState<C>
where
    C: BucketClient,
{
    pub fn new(storage_client: C) -> Self {
        Self { storage_client }
    }
}
