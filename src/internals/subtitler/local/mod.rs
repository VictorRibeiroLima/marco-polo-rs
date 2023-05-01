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
impl SubtitlerClient for LocalClient {
    fn estimate_time<BC: BucketClient>(&self, _: &SrtPayload, _: &BC) -> u32 {
        1000
    }

    async fn subtitle<BC: BucketClient + Sync>(
        &self,
        payload: &SrtPayload,
        bucket_client: &BC,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let video_id = payload.video_id.to_string();
        let video_uri = format!("videos/raw/{}.{}", video_id, "mp4"); // for now, we only support mp4,refactor later
        let video = bucket_client.download_file(&video_uri).await?;
        let srt = bucket_client.download_file(&payload.srt_uri).await?;
        let temp_dir = util::create_temp_dir()?;
        let temp_file_paths = util::write_to_temp_files(&video, &srt, &temp_dir, &video_id)?;

        util::call_ffmpeg(
            &temp_file_paths[0],
            &temp_file_paths[1],
            &temp_file_paths[2],
        )?;
        let output = util::read_output_file(&temp_file_paths[2])?;
        util::delete_temp_files(temp_file_paths)?;
        util::upload_output_file(bucket_client, output, &video_id).await?;
        Err("pior que foi".into())
    }
}
