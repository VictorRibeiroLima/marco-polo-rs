use std::{fs::File, io::Write, path::PathBuf, process::Command};

use crate::internals::cloud::traits::BucketClient;

pub fn create_temp_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = std::env::current_dir().unwrap();
    let temp_dir = root.join("temp");
    std::fs::create_dir_all(&temp_dir)?;
    Ok(temp_dir)
}

pub fn write_to_temp_files(
    video: &[u8],
    srt: &[u8],
    temp_dir: &PathBuf,
    id: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let video_path = temp_dir.join(format!("input_{}.{}", id, "mp4"));
    let srt_path = temp_dir.join(format!("{}.{}", id, "srt"));
    let output_path = temp_dir.join(format!("output_{}.{}", id, "mp4"));
    let mut temp_file_paths = Vec::new();

    let mut video_file = File::create(&video_path)?;
    video_file.write_all(&video)?;

    let mut srt_file = File::create(&srt_path)?;
    srt_file.write_all(&srt)?;

    temp_file_paths.push(video_path);
    temp_file_paths.push(srt_path);
    temp_file_paths.push(output_path);

    Ok(temp_file_paths)
}

pub fn call_ffmpeg(
    video_path: &PathBuf,
    srt_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&video_path)
        .arg("-vf")
        .arg(format!("subtitles={}", &srt_path.to_str().unwrap()))
        .arg(&output_path)
        .output()?;

    match output.status.code() {
        Some(0) => {}
        Some(_) => {
            println!("1:{:?}", output);
            return Err("ffmpeg failed".into());
        }
        None => {
            println!("2:{:?}", output);
            return Err("ffmpeg failed".into());
        }
    }

    Ok(())
}

pub fn read_output_file(output_path: &PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let output_bytes = std::fs::read(&output_path)?;
    Ok(output_bytes)
}

pub async fn upload_output_file<BC: BucketClient + Sync>(
    bucket_client: &BC,
    file: Vec<u8>,
    video_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_uri = format!("videos/processed/{}.{}", video_id, "mp4");
    bucket_client.upload_file(&output_uri, file).await?;

    Ok(())
}

pub fn delete_temp_files(temp_file_paths: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    for path in temp_file_paths {
        std::fs::remove_file(path)?;
    }

    Ok(())
}
