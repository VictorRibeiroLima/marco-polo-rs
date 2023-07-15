use crate::{
    database::models::video::VideoWithStorage,
    internals::{cloud::traits::BucketClient, ServiceProvider},
    util::fs::create_temp_dir,
};

use super::traits::SubtitlerClient;
use async_trait::async_trait;
mod util;
use crate::util::ffmpeg::subtitle_video_to_file;

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
        5000
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

        match subtitle_video_to_file(
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
        let to_delete = temp_file_paths[0..2].to_vec();
        util::delete_temp_files(to_delete)?;
        Ok(None)
    }
}
