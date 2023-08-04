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

    fn get_video_duration(&self, url: &str) -> Result<String, SyncError> {
        let output = Command::new("yt-dlp")
            .arg("--skip-download")
            .arg("--get-duration")
            .arg(url)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);

            println!(
                "Video duration estimation failed. Error message: {}",
                error_message
            );
            return Err(error_message.into());
        }

        let duration = String::from_utf8_lossy(&output.stdout);
        Ok(duration.to_string())
    }

    fn parse_duration(duration_str: &str) -> usize {
        let time_parts: Vec<&str> = duration_str.split(':').collect();
        if time_parts.len() == 3 {
            let hours = time_parts[0].parse::<usize>().unwrap_or(0);
            let minutes = time_parts[1].parse::<usize>().unwrap_or(0);
            let seconds = time_parts[2].parse::<usize>().unwrap_or(0);

            return hours * 3600 + minutes * 60 + seconds;
        }
        0
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
            .arg("bestvideo[height<=1080][fps<=30]+bestaudio/best[height<=1080][fps<=30]")
            .arg("--merge-output-format")
            .arg(format)
            .arg(url)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            println!("Video download failed. Error message: {}", error_message);
            return Err(error_message.into());
        }

        Ok(output_file)
    }

    async fn estimate_time(&self, url: &str) -> Result<usize, SyncError> {
        let duration = self.get_video_duration(url)?;
        let duration = Self::parse_duration(&duration);
        let estimated_time = duration * 2;
        if estimated_time < 4000 {
            return Ok(4000);
        } else if estimated_time > 43200 {
            return Ok(43200);
        }
        Ok(estimated_time)
    }
}
