use std::process::Command;

use crate::{database::models::video_storage::VideoFormat, util::fs::create_temp_dir, SyncError};

use super::traits::YoutubeDownloader;
use async_trait::async_trait;

#[derive(Clone)]
pub struct YtDl;

impl YtDl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl YoutubeDownloader for YtDl {
    async fn download(&self, url: &str) -> Result<String, SyncError> {
        let format: String = VideoFormat::Mkv.into();

        let video_id = uuid::Uuid::new_v4();

        let temp_dir = create_temp_dir()?;

        let output_file = format!("{}/{}.{}", temp_dir.to_str().unwrap(), video_id, format);

        let mut cmd = Command::new("yt-dlp");

        let output = cmd
            .arg("-o")
            .arg(&output_file)
            .arg("-f")
            .arg("bestvideo+bestaudio")
            .arg("--merge-output-format")
            .arg(format)
            .arg(url)
            .output()?;

        if !output.status.success() {
            println!(
                "Video download failed. Error message: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err("Failed to download video".into());
        }

        Ok(output_file)
    }
}
