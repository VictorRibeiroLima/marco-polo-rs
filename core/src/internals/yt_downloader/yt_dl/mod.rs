use std::process::Command;

use crate::{database::models::video_storage::VideoFormat, util::fs::create_temp_dir};

use super::traits::{YoutubeDownloader, YoutubeVideoConfig};
use async_trait::async_trait;
use uuid::Uuid;

pub struct YtDl;

#[async_trait]
impl YoutubeDownloader for YtDl {
    async fn download(
        &self,
        config: YoutubeVideoConfig,
    ) -> Result<(Vec<u8>, Uuid), Box<dyn std::error::Error>> {
        let format: String = match config.format {
            Some(format) => format.into(),
            None => VideoFormat::Mkv.into(),
        };
        let start_time = match config.start_time {
            Some(start_time) => start_time,
            None => "00:00:00".to_string(),
        };

        let mut cut = format!("-ss {}", start_time);

        if let Some(end_time) = config.end_time {
            cut.push_str(&format!(" -to {}", end_time));
        }

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
            .arg("--postprocessor-args")
            .arg(cut)
            .arg(&config.url)
            .output()?;

        if !output.status.success() {
            println!(
                "Video download failed. Error message: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err("Failed to download video".into());
        }

        let file = std::fs::read(&output_file)?;

        std::fs::remove_file(&output_file)?;

        Ok((file, video_id))
    }
}
