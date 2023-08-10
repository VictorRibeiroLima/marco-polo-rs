use std::sync::Arc;

use marco_polo_rs_core::internals::cloud::models::payload::PayloadType;
use tokio::sync::Mutex;

use crate::Message;

pub mod heavy;
pub mod light;

#[async_trait::async_trait]
pub trait Worker: Sized + Send + Sync + 'static {
    async fn handle(
        self,
        message: (Message, PayloadType),
        inactive_worker_pool: Arc<Mutex<Vec<Self>>>,
    );
}
