use crate::{
    database::models::video::VideoWithStorage,
    internals::{cloud::traits::BucketClient, ServiceProvider},
    util::fs::create_temp_dir,
};

use super::traits::SubtitlerClient;
use async_trait::async_trait;
mod util;

#[derive(Clone)]
pub struct LocalClient;

impl LocalClient {
    pub fn new() -> Self {
        println!("Creating LocalClient...");
        Self {}
    }
}

impl ServiceProvider for LocalClient {
    fn id(&self) -> i32 {
        1
    }
}

#[async_trait]
impl<BC: BucketClient> SubtitlerClient<BC> for LocalClient {
    fn estimate_time(&self, _: &VideoWithStorage, _: &BC) -> u32 {
        1000
    }

    async fn subtitle(
        &self,
        video: &VideoWithStorage,
        bucket_client: &BC,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Sync + Send>> {
        let video_id = video.video.id.to_string();
        let temp_dir = create_temp_dir()?;
        let temp_file_paths =
            util::write_to_temp_files(bucket_client, &temp_dir, &video_id).await?;

        match util::call_ffmpeg(
            &temp_file_paths[0],
            &temp_file_paths[1],
            &temp_file_paths[2],
        ) {
            Ok(_) => {}
            Err(e) => {
                util::delete_temp_files(temp_file_paths)?;
                return Err(e);
            }
        }
        match util::upload_output_file(bucket_client, &temp_file_paths[2], &video_id).await {
            Ok(_) => {}
            Err(e) => {
                util::delete_temp_files(temp_file_paths)?;
                return Err(e);
            }
        }
        util::delete_temp_files(temp_file_paths)?; // Invert this to delete first, then upload when the upload method start using a Vec<u8> instead of a PathBuf
        Ok(None)
    }
}
