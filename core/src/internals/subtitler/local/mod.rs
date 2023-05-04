use crate::internals::{
    cloud::{models::payload::SrtPayload, traits::BucketClient},
    ServiceProvider,
};

use super::traits::SubtitlerClient;
use async_trait::async_trait;
mod util;

pub struct LocalClient;

impl LocalClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl ServiceProvider for LocalClient {
    fn id() -> i32 {
        1
    }
}

#[async_trait]
impl<BC: BucketClient> SubtitlerClient<BC> for LocalClient {
    fn estimate_time(&self, _: &SrtPayload, _: &BC) -> u32 {
        1000
    }

    async fn subtitle(
        &self,
        payload: &SrtPayload,
        bucket_client: &BC,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let video_id = payload.video_id.to_string();
        let video_uri = format!("videos/raw/{}.{}", video_id, "mkv"); // for now, we only support mkv,refactor later
        let video = bucket_client.download_file(&video_uri).await?;
        let srt = bucket_client.download_file(&payload.srt_uri).await?;
        let temp_dir = util::create_temp_dir()?;
        let temp_file_paths = util::write_to_temp_files(&video, &srt, &temp_dir, &video_id)?;

        util::call_ffmpeg(
            &temp_file_paths[0],
            &temp_file_paths[1],
            &temp_file_paths[2],
        )?;
        util::upload_output_file(bucket_client, &temp_file_paths[2], &video_id).await?;
        util::delete_temp_files(temp_file_paths)?; // Invert this to delete first, then upload when the upload method start using a Vec<u8> instead of a PathBuf
        Ok(())
    }
}
