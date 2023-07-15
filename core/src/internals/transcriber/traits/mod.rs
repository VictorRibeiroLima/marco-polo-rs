use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{internals::ServiceProvider, SyncError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sentence {
    pub start_time: i32,
    pub end_time: i32,
    pub text: String,
}

#[async_trait]
pub trait TranscriberClient: ServiceProvider {
    async fn transcribe(&self, media_url: &str) -> Result<String, SyncError>;
    async fn transcribe_from_file(&self, file_path: &str) -> Result<String, SyncError>;
    async fn get_transcription_sentences(
        &self,
        transcription_id: &str,
    ) -> Result<Vec<Sentence>, SyncError>;
    async fn pool(&self, transcription_id: &str) -> Result<(), SyncError>;
}
